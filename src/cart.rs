/// Cart Chunk, or Scott Chunk
#[derive(Debug)]
pub struct CartChunk {
    /// scott_tag is always scot
    pub scott_tag:          [u8; 4],    // char     scott[4]

    /// scotsize should be 424
    /// See final `fillout` field below.
    pub scotsize:           u64,        // long     scotsize

    /// "scratchpad" area used by program. Should be ZERO
    pub alter:              u8,         // char     alter

    /// attrib is interpreted as eight flag bits
    /// bit0:    this file is not audio but parent of (obsolete) rotation set
    /// bit1:    length field interpretation (see `asclen` below). Store: 1
    /// bit2:    format of `asclen`: if set, is HMMSS, else: MM:SS
    ///          (for use when file is longer than 99:59)
    /// bit3:    this file is VOICE TRACK
    /// bit4:    this file is a rotation look-up table for new rotation scheme
    /// bit5     reserved
    /// bit6:    this file has segments and a segment order to play
    /// bit7:    indicates a valid `attrib2` field for additional flags.
    ///          if not set, `attrib2` is not reliable.
    ///          if application sets this bit, they MUST ensure `attrib2`
    ///          is clear of all garbage values
    pub attrib:             u8,         // US char  attrib

    /// "scratchpad" area to track position of artist. Store: ZERO
    pub artnum:             i16,        // short    artnum

    /// name of spot or title of song. pad with spaces   
    pub name:               [u8; 43],   // char     name[43]

    /// ascii copy/cart number (e.g. "1234" or "AZ22")
    pub copy:               [u8; 4],    // char     copy[4]

    /// future expansion of copy/cart. Store: SPACE
    pub padd:               u8,         // char     padd[1]

    /// `asclen` expressed in ascii (e.g. " 2:30") maximum of "99:59"
    /// note the front-padded space so the colon is always in the same location
    ///
    /// `attrib bit1 = 0`:   this length represents the time until the end audio file
    ///                      considered obsolete
    ///
    /// `attrib bit1 = 1`:   this length represents the time to beginning of EOM.
    /// 
    /// `attrib bit2 = 0`:   format: MM:SS
    /// `attrib bit2 = 1`:   format: HMMSS
    pub asclen:             [u8; 5],    // char     asclen[5]

    /// ---- START and END Seconds ---- //
    /// `start_seconds` and `start_hundreths` are considered one number stored separately
    /// e.g. 12.9 will be stored as 
    /// start_seconds = 12
    /// start_hundreths = 90
    /// START represents the an offset from the beginning of the file
    /// to the beginning of the usable audio (cue-in point).
    /// Also known as INTRO
    pub start_seconds:      i16,        // short    startseconds
    /// 
    pub start_hundreths:    i16,        // short    starthundreths

    /// Simalarly, `end_seconds` and `end_hundreths` represent the distance to the
    /// END of audio. the value stored represent the length of usable audio
    /// i.e. from the jump-in point (START) to the END of the audio portion of the file.
    /// Also known as EOD
    pub end_seconds:        i16,        // short    endseconds
    /// 
    pub end_hundreths:      i16,        // short    endhundreths
    /// 
    ///   marker          marker
    ///     \/              \/
    /// | INTRO | ------- | EOD |
    ///              ^
    ///      this length = EOD
    ///
    /// There could be audio preceeding INTRO and/or audio following the EOD marker

    /// ---- DATES ---- //
    /// Date format MMDDYY
    /// start date can be 000000 if not used
    /// end date can be 999999 if not used
    pub start_date:         [u8; 6],    // char     sdate[6]
    /// 
    pub kill_date:          [u8; 6],    // char     kdate[6]

    /// ---- HOURS ---- //
    /// Start and End hours are stored as the values 0 through 23 in a single byte
    /// with the highest bit set ( hr + 128 ). All values smaller than 128 + 1 (1am)
    /// including 128 + 0 (12m) are considered to be "all hours of the day"
    /// and will cause a display of ALL
    pub start_hour:         u8,         // char     startHR
    /// 
    pub kill_hour:          u8,         // char     killHR

    /// [ 'D' = digital recording | 'A' = analog recording ]
    /// (it's been my experience that it's always been analog.)
    pub digital:            u8,         // char     digital

    /// sample rate divided by 100, e.g. 441 = 44100
    pub sample_rate:        i16,        // short    samplerate

    /// [ 'S' = stereo | 'M' = mono ]
    pub stereo:             u8,         // char     stereo

    /// `compress` uses various values to indicate compression types.
    /// APT card:
    /// 251: compressed (aptx)
    /// 237: not compressed
    /// ANTEX card:
    /// 0:  PCM16
    /// 1:  PCM8
    /// 2:  PCMU8
    /// 3:  ADPCM1
    /// 4:  ADPCME
    /// 5:  CDIB
    /// 6:  CDIC
    /// 7:  DOLBYAC2
    /// 8:  MPEG
    /// 9:  APTX
    /// 10: WAVE
    /// It has been my experience that all files have use `10` here
    pub compress:           u8,         // US char  compress

    /// ---- EOM ---- //
    /// `eom[strt|len]` describe the position and length of EOM
    /// `eomstrt` length in tenths from audio beginning to start of EOM
    /// `eomlen` lenth of EOM in hendreths
    pub eomstrt:            i64,        // long     eomstrt
    /// 
    pub eomlen:             i16,        // short    eomlen

    /// ---- Extended Attributes ---- //
    /// `attrib2` is only valid if bit7 of `attrib` is set
    /// If bit7 is set, `attrib2` must be ZERO except those in use.
    /// bit0: do not play this file on the internet
    /// bit1: valid VTEOMOVR
    /// bit2: use `desired length` field (stretch + squeeze)
    /// bit3: if set, the 4 trigger[n] fields contain valid data
    /// bit4: this file contains valid hook-mode values
    /// bit5: delete this file after one play
    /// bit6: archive this file after one play
    /// bit7: this file originated as a netcatch/dtmf recording
    /// bit8: the `hrcanplay` (dayparting) bit-array is valid
    pub attrib2:            u32,        // US long  attrib2

    /// ---- Hooks ---- //
    /// Disclaimer: I have never seen hooks used.
    /// I have only ever seen these fields as `0` bytes.
    /// if bit4 of `attrib2` is set, the hook values are valid.
    /// They represent the jump-in, jump-out, and EOM timings
    /// of file playback for "hook" mode.
    /// `hookstartMS`:   milliseconds from start of file
    /// `hookeomMS`:     milliseconds from start of hook to eom of hook
    /// `hookendMS`:     milliseconds from start of hook to end of hook
    pub hookstart_ms:        u32,        // US long  hookstartMS
    /// 
    pub hookeom_ms:          u32,        // US long  hookmeomMS
    /// 
    pub hookend_ms:          u32,        // US long  hookendMS

    /// ---- Cat Colors ---- //
    /// I've never seen these used.
    /// Both `catfontcolor` and `catcolor` Store: ZERO
    pub catfontcolor:       u32,        // US long  catfontcolor
    /// 
    pub catcolor:           u32,        // US long  catcolor

    /// location of EOM when dealing with a file containing internal segments
    /// Store: ZERO
    pub segeompos:          i64,        // long     segeompos

    /// ---- Voice Track Support ---- //
    /// These two values represent a single value:
    /// the EOM override point
    /// if ZEROS, the normal EOM of the source before the VT is used.
    /// else; the regular EOM point is ignored and this is used to start the VT
    pub vt_start_secs:        i16,        // short    VTstartsecs
    /// 
    pub vt_start_hunds:       i16,        // short    VTstarthunds
    
    /// ---- Prior/Post ---- //
    /// Depreciated. Definition from docs:
    /// this is the CAT/CART number of the previous source,
    /// used to verify linkage to the VT. If filled with SPACES,
    /// no check is made and the VT is allowed to follow and event.
    /// If VT is following the wrong event, it is not allowed to play.
    /// Store: Spaces
    pub priorcat:           [u8; 3],    // char     priorcat[3]
    /// 
    pub priorcopy:          [u8; 4],    // char     priorcopy[4]
    /// 
    pub priorpadd:          u8,         // char     priorpadd[1]
    /// 
    pub postcat:            [u8; 3],    // char     postcat[3]
    /// 
    pub postcopy:           [u8; 4],    // char     postcopy[4]
    /// 
    pub postpadd:           u8,         // char     postpadd[1]

    /// This is an array of 168 bits representing every hour in the week.
    /// The first bit (MSB) of the first byte
    /// is the first hour of the week (Sunday, midnight to 1am).
    /// if bit8 of `attrib2` is set, then these bits are activated.
    /// If a bit is set, then the audio file is allowed to play
    /// in the hour of the week represented by that bit.
    pub hrcanplay:          [u8; 21],   // US char  hrcanplay[21]

    /// Reserved for future use. Fill with binary ZEROS
    pub future2:            [u8; 108],  // char     future[108]

    /// ---- Event information ---- //
    /// Fill with SPACES when not used
    /// `artist` and `trivia` must be padded with SPACES
    pub artist:             [u8; 34],   // char     artist[34]
    /// padded with SPACES
    pub trivia:             [u8; 34],   // char     trivia[34]
    /// acsii value of talk-up time (if song), e.g. "12"
    pub intro:              [u8; 2],    // char     intro[2]
    /// ascii character indicating nature of song ending
    /// F:
    /// C:
    pub end:                u8,         // char     end[1]
    /// ascii digits YYYY ("1984")
    pub year:               [u8; 4],    // char     year[4]

    /// Store: ZERO
    pub obsolete2:          u8,         // char     obsolete2
    
    /// Hour event was recorded (0 -> 23) + 128
    pub record_hour:              u8,         // char     recHR
    
    /// Date event was recorded fmt: MMDDYY 
    pub rdate:              [u8; 6],    // char     rdate[6]
    
    /// bitrate for an MPEG file divided by 1000
    pub mpegbitrate:        i16,        // short    mpegbitrate

    /// ---- Pitch/Playlevel ---- //
    /// consider these obsolete. Old docs included:
    ///
    /// percent of samplerate for playback (stored times 10)
    /// e.g.  1 percent faster = 1010   (means 101.0 %)
    /// e.g.  2 percent slower =  980   (means  98.0 %)
    /// two highest bits are flags:  bit15 = valid data
    ///                              bit14 = future flag
    /// if highest bit not set, data not valid, don't use.
    /// next-to-highest-bit reserved for possible over-ride flag
    /// (be sure to shave off two highest bits to use pitch)
    /// The lower 15 bits range from 0 to 32767 which
    /// represent using loudest volume the adapter is capable of
    /// down to OFF.  The value should be not be used as is, but
    /// as a ratio converted as appropriate to the number range
    /// used by the target adapter.  The highest bit is set when
    /// data is valid.  Strip off the highest bit and use the
    /// value.  If highest bit is not set, do not use the value
    /// (supply a default level instead).  NOTE: This scheme is
    /// will soon be considered the "old style".  If the special
    /// value of 21,845 (with highest bit NOT set) appears in this
    /// location, the location called "playpcent" should be used to
    /// set the playlevel by percent.  [See below.]
    pub pitch:              u16,        // Ushort   pitch
    /// see above                                   
    pub playlevel:          u16,        // Ushort   playlevel

    /// if high bit is set, next four bytes are valid
    /// plus the following seven bits are valid as flags
    /// (the lower seven bits will become flags and must be set 0
    /// if highest bit is set.)  The next highest bit will
    /// probably control the interpretation of the highest
    /// bit in playlevel.  If valid but off, it is a validation bit
    /// but if valid and on, the high playlevel can indicate %100+
    pub lenvalid:           u8,         // Uchar    lenvalid

    /// absolute total length of file in bytes at time of recording
    /// Used to verify during upload/download
    pub filelength:         u32,        // US long  filelength

    /// this contains the percent value of level adjustment
    /// that supercedes the "playlevel" value, when available.
    /// The highest bit is set to indicate the value is valid.
    /// To use, strip off the highest bit and consider the
    /// remaining value to be in tenths of a percent
    /// (i.e. 1000 = 100%).  It should be applied to the
    /// "standard" playback level.
    pub newplaylev:         u16,        // Ushort   newplaylev

    /// hundeths seconds removed from audio middle
    /// high bit set when valid
    pub chopsize:           u32,        // US long  chopsize

    /// milliseconds to subtract from precut EOM
    pub vteomovr:           u32,        // US long  vteomovr

    /// contains an override length (in hundredths of secs) that
    /// this event should be forced to play back within (from
    /// audio beginning to EOM point) if "stretch+squeeze"
    /// capabilities are available. This field is valid ONLY if
    /// bit2 of `attrib2` is set. ZERO means no "squeezing" is
    /// called for.
    pub desiredlen:         u32,        // US long  desiredlen

    /// ---- Triggers ---- //
    /// Triggers are generally not used anymore. There used
    /// to be 4 triggers, but newer conventions use the last trigger
    /// as a category. Old docs:
    /// when bit3 of `attrib2` is set, these four integers
    /// contain lengths (in tenths of seconds, in the three lower
    /// bytes) from the beginning of audio to trigger points
    /// within the audio.  The highest byte contains an
    /// ID number (1-255) that refers to the source of the
    /// trigger. The lengths are expected to be in order from
    /// earliest trigger to last. If not used, the integer MUST
    /// contain the value ZERO.
    pub trigger1:           u32,        // US long  triggers[4]
    
    /// 
    pub trigger2:           u32,        //
    /// 
    pub trigger3:           u32,        //
    /// 
    pub category:           [u8; 4],    //

    /// structure should be filled out with BINARY ZEROS to
    /// either a total size of 424 if part of a "fixed-RIFF"
    /// header or 512 if the header is stand-alone (i.e. not
    /// within a RIFF-style chunk)
    pub fillout:            [u8; 33],   // char     fillout[33]
}

