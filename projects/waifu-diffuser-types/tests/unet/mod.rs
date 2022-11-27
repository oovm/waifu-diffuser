use resource_path::ResourcePath;

use waifu_diffuser_types::{Text2ImageTask, UNetModel};

use crate::{here, save_json};

#[test]
fn anything_model() {
    let path = ResourcePath::new(
        "https://huggingface.co/oovm/anything/resolve/main/anything-v4.5-fp16/unet.onnx",
        "anything-v4.5-fp16.unet",
    )
    .unwrap();
    let mut net = UNetModel::new("anything-v4.5-fp16", path);
    net.add_example(Text2ImageTask::default().with_prompts("masterpiece, best quality, 1girl, white hair, medium hair, cat ears, closed eyes, looking at viewer, :3, cute, scarf, jacket, outdoors, streets",""));
    save_json(net, &here("unet/anything-v4.5-fp16.json")).unwrap();
}
