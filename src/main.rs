use dxgcap::DXGIManager;

fn main() {
    let screen = &mut DXGIManager::new(1000 * 60).unwrap();
    let temp = screen.capture_frame().unwrap();
    println!("each frame of the monitor has: {:?} bytes", temp.0.len());
    println!("its resolution is: {:?} * {:?}", temp.1.0, temp.1.1);
}
