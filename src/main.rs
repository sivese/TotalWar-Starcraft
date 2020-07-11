/*
        STW - super total war.

    It's just copy of Total War series. No more, no less.

    !! Device driver must support vulkan 1.2, or it will occur error message.
    !! Also it need Vulkan SDK (also Shaderc) to run vulkano
    !! Extension option is important. Pay attention to extension option codes.

    =========================== To Do ===========================
    Next example - https://vulkano.rs/guide/descriptor-sets , about shader, pipelining
 */

use vulkano::instance::{
    Instance
    ,InstanceExtensions
    ,PhysicalDevice
};

use vulkano::device::{
    Device
    ,DeviceExtensions
    ,Features
};

use vulkano::buffer::{
    BufferUsage
    ,CpuAccessibleBuffer
};

use vulkano::command_buffer::{
    AutoCommandBufferBuilder
    ,CommandBuffer
};

use std::sync::Arc;
use vulkano::sync::GpuFuture;
use vulkano::pipeline::ComputePipeline;

struct Dum {
    num:i32,
    chk:bool
}

fn main() {
    let _instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("Failed to create instance");

    let _pd = PhysicalDevice::enumerate(&_instance)
        .next().expect("no device available");

    for f in _pd.queue_families() {
        println!("Queue family with {:?} queue(s)", f.queues_count());
    }

    let _qf = _pd.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Couldn't find a graphical queue family");

    let (_dev, mut _qu) = {
        Device::new( _pd, &Features::none(), &DeviceExtensions {
            khr_storage_buffer_storage_class: true,
            ..DeviceExtensions::none()
        },
        [(_qf, 0.5)].iter().cloned() ).expect("failed to create device")
    };

    let _q = _qu.next().unwrap();

    let _data:i32 = 12;
    let _buf = CpuAccessibleBuffer::from_data(_dev.clone(), BufferUsage::all(), false, _data)
        .expect("Failed to create buffer!");

    let _data = Dum{ num: 12, chk: true };
    let _buf = CpuAccessibleBuffer::from_data(_dev.clone(), BufferUsage::all(), false, _data)
        .unwrap();

/*
    let _it = (0..128).map(|_| 5u8);
    let _buf = CpuAccessibleBuffer::from_data(_dev.clone(), BufferUsage::all(), false, _it)
        .unwrap();

    let mut _cont = _buf.write().unwrap();

    _cont[10] = 80;
    _cont[6] = 12;
*/

    let mut _cont = _buf.write().unwrap();

    _cont.num *= 5;
    _cont.chk = false;

    let _sc = 0 .. 64;
    let _src = CpuAccessibleBuffer::from_iter(_dev.clone(), BufferUsage::all(), false,
                                                _sc).expect("Dailed to create buffer");

    let _dc = (0 .. 64).map(|_| 0);
    let _dest = CpuAccessibleBuffer::from_iter(_dev.clone(), BufferUsage::all(), false,
                                              _dc).expect("Failed to create buffer");

    let mut _bd = AutoCommandBufferBuilder::new(_dev.clone(), _q.family()).unwrap();
    _bd.copy_buffer(_src.clone(), _dest.clone()).unwrap();

    let _cb = _bd.build().unwrap();
    let _f = _cb.execute(_q.clone()).unwrap();
    _f.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let _sc = _src.read().unwrap();
    let _dc = _dest.read().unwrap();
    assert_eq!(&*_sc, &*_dc);

    let _di = 0..65536;
    let _db = CpuAccessibleBuffer::from_iter(_dev.clone(), BufferUsage::all(), false, _di)
        .expect("Failed to create buffer");

    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            src: "
#version 450
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;
void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 12;
}"
        }
    }

    let _shd = cs::Shader::load(_dev.clone())
        .expect("Failed to create shader module");

    let _cp = Arc::new(ComputePipeline::new(_dev.clone(), &_shd.main_entry_point(), &())
        .expect("Failed to create compute pipeline"));
}
