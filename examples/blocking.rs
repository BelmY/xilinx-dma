use anyhow::Result;
use xilinx_dma::DmaBuff;
use xilinx_dma::AxiDma;

fn main() -> Result<()> {

    let dma_buffer_h2d = DmaBuff::new("udmabuf0")?;
    let dma_buffer_d2h = DmaBuff::new("udmabuf1")?;
    println!("{:?}", dma_buffer_h2d);
    println!("{:?}", dma_buffer_d2h);

    let slice_h2d = dma_buffer_h2d.slice::<u32>();
    let slice_d2h = dma_buffer_d2h.slice::<u32>();

    for i in slice_d2h.iter_mut() {
        *i = 0;
    }

    for i in slice_h2d.iter_mut() {
        *i = fastrand::u32(0..1024);
    }

    let mut dma_h2d = AxiDma::new("uio4")?;
    let mut dma_d2h = AxiDma::new("uio5")?;
    println!("{:?}", dma_h2d);
    println!("{:?}", dma_d2h);

    dma_h2d.start_h2d(&dma_buffer_h2d, 1024*4);
    dma_d2h.start_d2h(&dma_buffer_d2h, 1024*4);
    println!("transfers started");

    dma_h2d.wait_h2d();
    println!("h2d done");
    dma_d2h.wait_d2h();
    println!("d2h done");

    for i in 0..1024 {
        assert_eq!(slice_d2h[i], slice_h2d[i] + 123);
    }

    Ok(())
}
