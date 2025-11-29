// The GUI parts conditioned by the "example-gui" feature
// depend on the gtk crate, which is an interface to the 
// native GTK libs.
// Although, the other parts which are not conditioned
// altogether demonstrate the basic use of this crate.

extern crate volt_waveform;

#[cfg(feature = "example-gui")]
extern crate gtk4;
#[cfg(feature = "example-gui")]
extern crate gdk_pixbuf;

use std::f64;

use volt_waveform::{
    SampleSequence,
    WaveformConfig,
    Color,
    BinnedWaveformRenderer,
    TimeRange,
};

#[cfg(feature = "example-gui")]
use gtk4::{Picture, Window, prelude::{
    GtkWindowExt, WidgetExt
}};
#[cfg(feature = "example-gui")]
use gdk_pixbuf::Pixbuf;

fn main() {
    let main_loop: gtk4::glib::MainLoop = gtk4::glib::MainLoop::new(None, false);

    #[cfg(feature = "example-gui")]
    {
        if gtk4::init().is_err() {
            panic!("Failed to initialize gtk4.");
        }
    }

    #[cfg(feature = "example-gui")]
    let window = Window::new();
    #[cfg(feature = "example-gui")]
    {
        window.set_title(Some("A simple waveform renderer test"));
        window.set_default_size(800, 100);
        let main_loop_clone = main_loop.clone();
        window.connect_close_request(move |win| {
            win.close();
            main_loop_clone.quit();
            gtk4::glib::Propagation::Proceed
        });
    }

    // Generate samples to show.
    let mut samples: Vec<f64> = Vec::new();
    for t in 0..44100 {
        samples.push(
            ((t as f64) / 100f64 * 2f64 * f64::consts::PI).sin() * ((t as f64) / 10000f64 * 2f64 * f64::consts::PI).sin(),
        );
    }

    // The renderer's config.
    let config = WaveformConfig::new(
        -1f64, // Minimum amplitude to show
        1f64, // Maximum amplitude to show

        // Foreground color
        Color::Vector4(0, 0, 0, 255),

        // Background color
        Color::Vector4(255, 255, 255, 255)
    ).unwrap();

    // Put a reference to the samples here along with its sample rate.
    // We need to set a sample rate because it will be used
    // when you specify the time range in seconds.
    let ss = SampleSequence {
        data: &samples[..],
        sample_rate: 44100f64,
    };

    // Construct the renderer.
    // The second argument is the bin size.
    // The renderer will bin the samples, and then
    // calculate the minimum and maximum amplitude values for
    // each bin.
    let wfg = BinnedWaveformRenderer::new(&ss, 10, config).unwrap();

    // Render!
    // The renderer doesn't look at the actual audio samples here.
    // Instead it will use the binned min/max values calculated above,
    // making the rendering quite faster.
    let vec: Vec<u8> =
        wfg.render_vec(TimeRange::Seconds(0.0f64, 1.0f64), (800, 100))
        .unwrap();

    #[cfg(feature = "example-gui")]
    {
        let pixbuf =
            Pixbuf::from_mut_slice(vec, gdk_pixbuf::Colorspace::Rgb, true, 8, 800, 100, 800 * 4);
        let picture = Picture::for_pixbuf(&pixbuf);
        picture.set_can_shrink(false);
        picture.set_size_request(800, 100);
        window.set_child(Some(&picture));
        window.present();
        main_loop.run();
    }
}