use std::sync::Arc;

use ambisonic::{Ambisonic, AmbisonicBuilder, SoundController};

use rodio::dynamic_mixer::{DynamicMixerController, mixer};

use specs::prelude::*;

pub struct Audio {
    pub ambisonic: Ambisonic
}

impl Default for Audio {
    fn default() -> Self {
        Audio {
            ambisonic: AmbisonicBuilder::default().build()
        }
    }
}

#[derive(Component)]
pub struct SoundEmitter {
    pub mixer_controller: Arc<DynamicMixerController<f32>>,
    pub spatial_controller: SoundController,
}

impl SoundEmitter {
    pub fn new(context: &Ambisonic) -> Self {
        let (mixer_controller, source) = mixer(1, 48000);
        let spatial_controller = context.play(source);

        SoundEmitter {
            mixer_controller,
            spatial_controller,
        }
    }
}
