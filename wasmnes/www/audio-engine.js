const MAX_OSC_VOLUME = 0.05;

export default function AudioEngine() {

    this.setPulseDutyCycle = function(osc, dc) {
        const numCoeffs = 128;
        let realCoeffs = new Float32Array(numCoeffs);
        let imagCoeffs = new Float32Array(numCoeffs);
        realCoeffs[0] = dc;
        for (var i = 1; i < numCoeffs; i++) {
            var npi = i * Math.PI;
            realCoeffs[i] = 4 / npi * Math.sin(npi * dc);
        }
        let wave = this.audioctx.createPeriodicWave(realCoeffs, imagCoeffs);
        osc.setPeriodicWave(wave);
    }

    this.audioctx = new (window.AudioContext || window.webkitAudioContext)();

    /**
     * All audio nodes
     */

    this.outputGainNode = this.audioctx.createGain();
    
    this.pulse1GainNode = this.audioctx.createGain();
    this.pulse2GainNode = this.audioctx.createGain();
    this.triangleGainNode = this.audioctx.createGain();

    this.pulse1OscNode = this.audioctx.createOscillator();
    this.pulse2OscNode = this.audioctx.createOscillator();
    this.triangleOscNode = this.audioctx.createOscillator();

    /**
     * Connect all audio nodes
     */

    this.outputGainNode.connect(this.audioctx.destination);
    
    this.pulse1GainNode.connect(this.outputGainNode);
    this.pulse2GainNode.connect(this.outputGainNode);
    this.triangleGainNode.connect(this.outputGainNode);

    this.pulse1OscNode.connect(this.pulse1GainNode);
    this.pulse2OscNode.connect(this.pulse2GainNode);
    this.triangleOscNode.connect(this.triangleGainNode);
 
    /**
     * Set default values
     */

    this.outputGainNode.gain.value = 1;
    this.pulse1GainNode.gain.value = 0;
    this.pulse2GainNode.gain.value = 0;
    this.triangleGainNode.gain.value = 0;

    this.setPulseDutyCycle(this.pulse1OscNode, 0.5);
    this.setPulseDutyCycle(this.pulse2OscNode, 0.5);
    this.triangleOscNode.type = "triangle";

    /**
     * Start oscillators
     */
    this.start = function() {
        this.pulse1OscNode.start();
        this.pulse2OscNode.start();
        this.triangleOscNode.start();
    }
     

     this.processApuCode = function(apu_code, apu_config) {
        if (apu_code == 0) {
            return;
        }

        switch (apu_code) {
            case 1:
                this.pulse1GainNode.gain.value = apu_config.en_pulse1 ? apu_config.pulse1_volume*MAX_OSC_VOLUME : 0;
                this.pulse2GainNode.gain.value = apu_config.en_pulse2 ? apu_config.pulse2_volume*MAX_OSC_VOLUME : 0;
                this.triangleGainNode.gain.value = apu_config.en_triangle ? MAX_OSC_VOLUME : 0;
            case 2:
                this.setPulseDutyCycle(this.pulse1OscNode, apu_config.pulse1_dutycycle);
                this.pulse1OscNode.frequency.value = apu_config.pulse1_frequency;
            case 3:
                this.pulse1GainNode.gain.value = apu_config.pulse1_volume*MAX_OSC_VOLUME;
            
            case 4:
                this.setPulseDutyCycle(this.pulse2OscNode, apu_config.pulse2_dutycycle);
                this.pulse2OscNode.frequency.value = apu_config.pulse2_frequency;
            case 5:
                this.pulse2GainNode.gain.value = apu_config.pulse2_volume*MAX_OSC_VOLUME;
            
            case 6:
                this.triangleOscNode.frequency.value = apu_config.triangle_frequency;
            case 7:
                this.triangleGainNode.gain.value = apu_config.triangle_muted ? 0 : MAX_OSC_VOLUME;
        
        }
     }

    
}

