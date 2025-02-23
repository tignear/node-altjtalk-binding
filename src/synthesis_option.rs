use htsengine::HTSEngine;

#[napi(object)]
pub struct SynthesisOption {
  /// Sampling frequency
  /// Must be integer. 1<=sampling_frequency.
  pub sampling_frequency: Option<u32>,

  /// Frame shift
  /// Must be integer. 1<=frame_period.
  pub frame_period: Option<u32>,

  /// Frequency warping parameter alpha
  /// 0.0<=all_pass_constant<=1.0.
  pub all_pass_constant: Option<f64>,

  /// Postfiltering coefficient parameter beta
  /// Default is 0.0. 0.0<=postfiltering_coefficient<=1.0.
  pub postfiltering_coefficient: Option<f64>,

  /// Speech speed
  /// Default is 1.0. 0<=speech_speed_rate. Warning: Do not set a very small value as it consumes CPU time.
  pub speech_speed_rate: Option<f64>,

  /// Additional half tone
  /// Default is 0.0.
  pub additional_half_tone: Option<f64>,

  /// MSD threshold for Stream #1
  /// Default is 0.5. 0.0<=voiced_unvoiced_threshold<=1.0.
  pub voiced_unvoiced_threshold: Option<f64>,

  /// GV weight for Stream #0
  /// Default is 1.0. 0.0<=weight_of_gv_for_spectrum.
  pub weight_of_gv_for_spectrum: Option<f64>,

  /// GV weight for Stream #1
  /// Default is 1.0. 0.0<=weight_of_gv_for_log_f0.
  pub weight_of_gv_for_log_f0: Option<f64>,

  /// Volume in dB
  /// Default is 0.0.
  pub volume_in_db: Option<f64>,
}

impl SynthesisOption {
  pub fn from_engine(engine: &HTSEngine) -> Self {
    Self {
      sampling_frequency: Some(engine.get_sampling_frequency() as u32),
      frame_period: Some(engine.get_fperiod() as u32),
      all_pass_constant: Some(engine.get_alpha()),
      postfiltering_coefficient: Some(engine.get_beta()),
      speech_speed_rate: None,
      additional_half_tone: None,
      voiced_unvoiced_threshold: Some(engine.get_msd_threshold(1)),
      weight_of_gv_for_spectrum: Some(engine.get_gv_weight(0)),
      weight_of_gv_for_log_f0: Some(engine.get_gv_weight(1)),
      volume_in_db: Some(engine.get_volume()),
    }
  }
  pub fn apply_to_engine(&self, engine: &mut HTSEngine, default: &Self) {
    engine.set_sampling_frequency(
      self
        .sampling_frequency
        .or(default.sampling_frequency)
        .unwrap_or(48000) as usize,
    );
    engine.set_fperiod(self.frame_period.or(default.frame_period).unwrap_or(240) as usize);
    engine.set_alpha(
      self
        .all_pass_constant
        .or(default.all_pass_constant)
        .unwrap_or(0.55),
    );
    engine.set_beta(self.postfiltering_coefficient.unwrap_or(0.));
    engine.set_speed(self.speech_speed_rate.unwrap_or(1.));
    engine.add_half_tone(self.additional_half_tone.unwrap_or(0.));
    engine.set_msd_threshold(1, self.voiced_unvoiced_threshold.unwrap_or(0.5));
    engine.set_gv_weight(0, self.weight_of_gv_for_spectrum.unwrap_or(1.));
    engine.set_gv_weight(1, self.weight_of_gv_for_log_f0.unwrap_or(1.));
    engine.set_volume(self.volume_in_db.unwrap_or(0.));
  }
}
