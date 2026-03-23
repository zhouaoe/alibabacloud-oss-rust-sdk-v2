use std::collections::HashMap;
use std::path::Path;

use mime_guess::from_ext;
use regex::Regex;

lazy_static::lazy_static! {
    /// https://github.com/abonander/mime_guess/blob/master/src/mime_types.rs
    static ref EXT_TO_MIME_TYPE: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            ".ppsx",
            "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
        );
        m.insert(
            ".pptx",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        );
        m.insert(
            ".sldx",
            "application/vnd.openxmlformats-officedocument.presentationml.slide",
        );
        m.insert(
            ".docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        );
        m.insert(
            ".dotx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
        );
        m.insert("xlam", "application/vnd.ms-excel.addin.macroEnabled.12");
        m.insert(
            ".xlsb",
            "application/vnd.ms-excel.sheet.binary.macroEnabled.12",
        );
        m.insert("apk", "application/vnd.android.package-archive");
        m.insert("hqx", "application/mac-binhex40");
        m.insert("cpt", "application/mac-compactpro");
        m.insert("doc", "application/msword");
        m.insert("ogg", "application/ogg");
        m.insert("pdf", "application/pdf");
        m.insert("rtf", "text/rtf");
        m.insert("mif", "application/vnd.mif");
        m.insert("xls", "application/vnd.ms-excel");
        m.insert("ppt", "application/vnd.ms-powerpoint");
        m.insert("odc", "application/vnd.oasis.opendocument.chart");
        m.insert("odb", "application/vnd.oasis.opendocument.database");
        m.insert("odf", "application/vnd.oasis.opendocument.formula");
        m.insert("odg", "application/vnd.oasis.opendocument.graphics");
        m.insert(
            ".otg",
            "application/vnd.oasis.opendocument.graphics-template",
        );
        m.insert("odi", "application/vnd.oasis.opendocument.image");
        m.insert("odp", "application/vnd.oasis.opendocument.presentation");
        m.insert(
            ".otp",
            "application/vnd.oasis.opendocument.presentation-template",
        );
        m.insert("ods", "application/vnd.oasis.opendocument.spreadsheet");
        m.insert(
            ".ots",
            "application/vnd.oasis.opendocument.spreadsheet-template",
        );
        m.insert("odt", "application/vnd.oasis.opendocument.text");
        m.insert("odm", "application/vnd.oasis.opendocument.text-master");
        m.insert("ott", "application/vnd.oasis.opendocument.text-template");
        m.insert("oth", "application/vnd.oasis.opendocument.text-web");
        m.insert("sxw", "application/vnd.sun.xml.writer");
        m.insert("stw", "application/vnd.sun.xml.writer.template");
        m.insert("sxc", "application/vnd.sun.xml.calc");
        m.insert("stc", "application/vnd.sun.xml.calc.template");
        m.insert("sxd", "application/vnd.sun.xml.draw");
        m.insert("std", "application/vnd.sun.xml.draw.template");
        m.insert("sxi", "application/vnd.sun.xml.impress");
        m.insert("sti", "application/vnd.sun.xml.impress.template");
        m.insert("sxg", "application/vnd.sun.xml.writer.global");
        m.insert("sxm", "application/vnd.sun.xml.math");
        m.insert("sis", "application/vnd.symbian.install");
        m.insert("wbxml", "application/vnd.wap.wbxml");
        m.insert("wmlc", "application/vnd.wap.wmlc");
        m.insert("wmlsc", "application/vnd.wap.wmlscriptc");
        m.insert("bcpio", "application/x-bcpio");
        m.insert("torrent", "application/x-bittorrent");
        m.insert("bz2", "application/x-bzip2");
        m.insert("vcd", "application/x-cdlink");
        m.insert("pgn", "application/x-chess-pgn");
        m.insert("cpio", "application/x-cpio");
        m.insert("csh", "application/x-csh");
        m.insert("dvi", "application/x-dvi");
        m.insert("spl", "application/x-futuresplash");
        m.insert("gtar", "application/x-gtar");
        m.insert("hdf", "application/x-hdf");
        m.insert("jar", "application/x-java-archive");
        m.insert("jnlp", "application/x-java-jnlp-file");
        m.insert("js", "application/x-javascript");
        m.insert("ksp", "application/x-kspread");
        m.insert("chrt", "application/x-kchart");
        m.insert("kil", "application/x-killustrator");
        m.insert("latex", "application/x-latex");
        m.insert("rpm", "application/x-rpm");
        m.insert("sh", "application/x-sh");
        m.insert("shar", "application/x-shar");
        m.insert("swf", "application/x-shockwave-flash");
        m.insert("sit", "application/x-stuffit");
        m.insert("sv4cpio", "application/x-sv4cpio");
        m.insert("sv4crc", "application/x-sv4crc");
        m.insert("tar", "application/x-tar");
        m.insert("tcl", "application/x-tcl");
        m.insert("tex", "application/x-tex");
        m.insert("man", "application/x-troff-man");
        m.insert("me", "application/x-troff-me");
        m.insert("ms", "application/x-troff-ms");
        m.insert("ustar", "application/x-ustar");
        m.insert("src", "application/x-wais-source");
        m.insert("zip", "application/zip");
        m.insert("m3u", "audio/x-mpegurl");
        m.insert("ra", "audio/x-pn-realaudio");
        m.insert("wav", "audio/x-wav");
        m.insert("wma", "audio/x-ms-wma");
        m.insert("wax", "audio/x-ms-wax");
        m.insert("pdb", "chemical/x-pdb");
        m.insert("xyz", "chemical/x-xyz");
        m.insert("bmp", "image/bmp");
        m.insert("gif", "image/gif");
        m.insert("ief", "image/ief");
        m.insert("png", "image/png");
        m.insert("wbmp", "image/vnd.wap.wbmp");
        m.insert("ras", "image/x-cmu-raster");
        m.insert("pnm", "image/x-portable-anymap");
        m.insert("pbm", "image/x-portable-bitmap");
        m.insert("pgm", "image/x-portable-graymap");
        m.insert("ppm", "image/x-portable-pixmap");
        m.insert("rgb", "image/x-rgb");
        m.insert("xbm", "image/x-xbitmap");
        m.insert("xpm", "image/x-xpixmap");
        m.insert("xwd", "image/x-xwindowdump");
        m.insert("css", "text/css");
        m.insert("rtx", "text/richtext");
        m.insert("tsv", "text/tab-separated-values");
        m.insert("jad", "text/vnd.sun.j2me.app-descriptor");
        m.insert("wml", "text/vnd.wap.wml");
        m.insert("wmls", "text/vnd.wap.wmlscript");
        m.insert("etx", "text/x-setext");
        m.insert("mxu", "video/vnd.mpegurl");
        m.insert("flv", "video/x-flv");
        m.insert("wm", "video/x-ms-wm");
        m.insert("wmv", "video/x-ms-wmv");
        m.insert("wmx", "video/x-ms-wmx");
        m.insert("wvx", "video/x-ms-wvx");
        m.insert("avi", "video/x-msvideo");
        m.insert("movie", "video/x-sgi-movie");
        m.insert("ice", "x-conference/x-cooltalk");
        m.insert("3gp", "video/3gpp");
        m.insert("ai", "application/postscript");
        m.insert("aif", "audio/x-aiff");
        m.insert("aifc", "audio/x-aiff");
        m.insert("aiff", "audio/x-aiff");
        m.insert("asc", "text/plain");
        m.insert("atom", "application/atom+xml");
        m.insert("au", "audio/basic");
        m.insert("bin", "application/octet-stream");
        m.insert("cdf", "application/x-netcdf");
        m.insert("cgm", "image/cgm");
        m.insert("class", "application/octet-stream");
        m.insert("dcr", "application/x-director");
        m.insert("dif", "video/x-dv");
        m.insert("dir", "application/x-director");
        m.insert("djv", "image/vnd.djvu");
        m.insert("djvu", "image/vnd.djvu");
        m.insert("dll", "application/octet-stream");
        m.insert("dmg", "application/octet-stream");
        m.insert("dms", "application/octet-stream");
        m.insert("dtd", "application/xml-dtd");
        m.insert("dv", "video/x-dv");
        m.insert("dxr", "application/x-director");
        m.insert("eps", "application/postscript");
        m.insert("exe", "application/octet-stream");
        m.insert("ez", "application/andrew-inset");
        m.insert("gram", "application/srgs");
        m.insert("grxml", "application/srgs+xml");
        m.insert("gz", "application/x-gzip");
        m.insert("htm", "text/html");
        m.insert("html", "text/html");
        m.insert("ico", "image/x-icon");
        m.insert("ics", "text/calendar");
        m.insert("ifb", "text/calendar");
        m.insert("iges", "model/iges");
        m.insert("igs", "model/iges");
        m.insert("jp2", "image/jp2");
        m.insert("jpe", "image/jpeg");
        m.insert("jpeg", "image/jpeg");
        m.insert("jpg", "image/jpeg");
        m.insert("kar", "audio/midi");
        m.insert("lha", "application/octet-stream");
        m.insert("lzh", "application/octet-stream");
        m.insert("m4a", "audio/mp4a-latm");
        m.insert("m4p", "audio/mp4a-latm");
        m.insert("m4u", "video/vnd.mpegurl");
        m.insert("m4v", "video/x-m4v");
        m.insert("mac", "image/x-macpaint");
        m.insert("mathml", "application/mathml+xml");
        m.insert("mesh", "model/mesh");
        m.insert("mid", "audio/midi");
        m.insert("midi", "audio/midi");
        m.insert("mov", "video/quicktime");
        m.insert("mp2", "audio/mpeg");
        m.insert("mp3", "audio/mpeg");
        m.insert("mp4", "video/mp4");
        m.insert("mpe", "video/mpeg");
        m.insert("mpeg", "video/mpeg");
        m.insert("mpg", "video/mpeg");
        m.insert("mpga", "audio/mpeg");
        m.insert("msh", "model/mesh");
        m.insert("nc", "application/x-netcdf");
        m.insert("oda", "application/oda");
        m.insert("ogv", "video/ogv");
        m.insert("pct", "image/pict");
        m.insert("pic", "image/pict");
        m.insert("pict", "image/pict");
        m.insert("pnt", "image/x-macpaint");
        m.insert("pntg", "image/x-macpaint");
        m.insert("ps", "application/postscript");
        m.insert("qt", "video/quicktime");
        m.insert("qti", "image/x-quicktime");
        m.insert("qtif", "image/x-quicktime");
        m.insert("ram", "audio/x-pn-realaudio");
        m.insert("rdf", "application/rdf+xml");
        m.insert("rm", "application/vnd.rn-realmedia");
        m.insert("roff", "application/x-troff");
        m.insert("sgm", "text/sgml");
        m.insert("sgml", "text/sgml");
        m.insert("silo", "model/mesh");
        m.insert("skd", "application/x-koan");
        m.insert("skm", "application/x-koan");
        m.insert("skp", "application/x-koan");
        m.insert("skt", "application/x-koan");
        m.insert("smi", "application/smil");
        m.insert("smil", "application/smil");
        m.insert("snd", "audio/basic");
        m.insert("so", "application/octet-stream");
        m.insert("svg", "image/svg+xml");
        m.insert("t", "application/x-troff");
        m.insert("texi", "application/x-texinfo");
        m.insert("texinfo", "application/x-texinfo");
        m.insert("tif", "image/tiff");
        m.insert("tiff", "image/tiff");
        m.insert("tr", "application/x-troff");
        m.insert("txt", "text/plain");
        m.insert("vrml", "model/vrml");
        m.insert("vxml", "application/voicexml+xml");
        m.insert("webm", "video/webm");
        m.insert("wrl", "model/vrml");
        m.insert("xht", "application/xhtml+xml");
        m.insert("xhtml", "application/xhtml+xml");
        m.insert("xml", "application/xml");
        m.insert("xsl", "application/xml");
        m.insert("xslt", "application/xslt+xml");
        m.insert("xul", "application/vnd.mozilla.xul+xml");
        m.insert("webp", "image/webp");
        m.insert("323", "text/h323");
        m.insert("aab", "application/x-authoware-bin");
        m.insert("aam", "application/x-authoware-map");
        m.insert("aas", "application/x-authoware-seg");
        m.insert("acx", "application/internet-property-stream");
        m.insert("als", "audio/X-Alpha5");
        m.insert("amc", "application/x-mpeg");
        m.insert("ani", "application/octet-stream");
        m.insert("asd", "application/astound");
        m.insert("asf", "video/x-ms-asf");
        m.insert("asn", "application/astound");
        m.insert("asp", "application/x-asap");
        m.insert("asr", "video/x-ms-asf");
        m.insert("asx", "video/x-ms-asf");
        m.insert("avb", "application/octet-stream");
        m.insert("awb", "audio/amr-wb");
        m.insert("axs", "application/olescript");
        m.insert("bas", "text/plain");
        m.insert("bin ", "application/octet-stream");
        m.insert("bld", "application/bld");
        m.insert("bld2", "application/bld2");
        m.insert("bpk", "application/octet-stream");
        m.insert("c", "text/plain");
        m.insert("cal", "image/x-cals");
        m.insert("cat", "application/vnd.ms-pkiseccat");
        m.insert("ccn", "application/x-cnc");
        m.insert("cco", "application/x-cocoa");
        m.insert("cer", "application/x-x509-ca-cert");
        m.insert("cgi", "magnus-internal/cgi");
        m.insert("chat", "application/x-chat");
        m.insert("clp", "application/x-msclip");
        m.insert("cmx", "image/x-cmx");
        m.insert("co", "application/x-cult3d-object");
        m.insert("cod", "image/cis-cod");
        m.insert("conf", "text/plain");
        m.insert("cpp", "text/plain");
        m.insert("crd", "application/x-mscardfile");
        m.insert("crl", "application/pkix-crl");
        m.insert("crt", "application/x-x509-ca-cert");
        m.insert("csm", "chemical/x-csml");
        m.insert("csml", "chemical/x-csml");
        m.insert("cur", "application/octet-stream");
        m.insert("dcm", "x-lml/x-evm");
        m.insert("dcx", "image/x-dcx");
        m.insert("der", "application/x-x509-ca-cert");
        m.insert("dhtml", "text/html");
        m.insert("dot", "application/msword");
        m.insert("dwf", "drawing/x-dwf");
        m.insert("dwg", "application/x-autocad");
        m.insert("dxf", "application/x-autocad");
        m.insert("ebk", "application/x-expandedbook");
        m.insert("emb", "chemical/x-embl-dl-nucleotide");
        m.insert("embl", "chemical/x-embl-dl-nucleotide");
        m.insert("epub", "application/epub+zip");
        m.insert("eri", "image/x-eri");
        m.insert("es", "audio/echospeech");
        m.insert("esl", "audio/echospeech");
        m.insert("etc", "application/x-earthtime");
        m.insert("evm", "x-lml/x-evm");
        m.insert("evy", "application/envoy");
        m.insert("fh4", "image/x-freehand");
        m.insert("fh5", "image/x-freehand");
        m.insert("fhc", "image/x-freehand");
        m.insert("fif", "application/fractals");
        m.insert("flr", "x-world/x-vrml");
        m.insert("fm", "application/x-maker");
        m.insert("fpx", "image/x-fpx");
        m.insert("fvi", "video/isivideo");
        m.insert("gau", "chemical/x-gaussian-input");
        m.insert("gca", "application/x-gca-compressed");
        m.insert("gdb", "x-lml/x-gdb");
        m.insert("gps", "application/x-gps");
        m.insert("h", "text/plain");
        m.insert("hdm", "text/x-hdml");
        m.insert("hdml", "text/x-hdml");
        m.insert("hlp", "application/winhlp");
        m.insert("hta", "application/hta");
        m.insert("htc", "text/x-component");
        m.insert("hts", "text/html");
        m.insert("htt", "text/webviewhtml");
        m.insert("ifm", "image/gif");
        m.insert("ifs", "image/ifs");
        m.insert("iii", "application/x-iphone");
        m.insert("imy", "audio/melody");
        m.insert("ins", "application/x-internet-signup");
        m.insert("ips", "application/x-ipscript");
        m.insert("ipx", "application/x-ipix");
        m.insert("isp", "application/x-internet-signup");
        m.insert("it", "audio/x-mod");
        m.insert("itz", "audio/x-mod");
        m.insert("ivr", "i-world/i-vrml");
        m.insert("j2k", "image/j2k");
        m.insert("jam", "application/x-jam");
        m.insert("java", "text/plain");
        m.insert("jfif", "image/pipeg");
        m.insert("jpz", "image/jpeg");
        m.insert("jwc", "application/jwc");
        m.insert("kjx", "application/x-kjx");
        m.insert("lak", "x-lml/x-lak");
        m.insert("lcc", "application/fastman");
        m.insert("lcl", "application/x-digitalloca");
        m.insert("lcr", "application/x-digitalloca");
        m.insert("lgh", "application/lgh");
        m.insert("lml", "x-lml/x-lml");
        m.insert("lmlpack", "x-lml/x-lmlpack");
        m.insert("log", "text/plain");
        m.insert("lsf", "video/x-la-asf");
        m.insert("lsx", "video/x-la-asf");
        m.insert("m13", "application/x-msmediaview");
        m.insert("m14", "application/x-msmediaview");
        m.insert("m15", "audio/x-mod");
        m.insert("m3url", "audio/x-mpegurl");
        m.insert("m4b", "audio/mp4a-latm");
        m.insert("ma1", "audio/ma1");
        m.insert("ma2", "audio/ma2");
        m.insert("ma3", "audio/ma3");
        m.insert("ma5", "audio/ma5");
        m.insert("map", "magnus-internal/imagemap");
        m.insert("mbd", "application/mbedlet");
        m.insert("mct", "application/x-mascot");
        m.insert("mdb", "application/x-msaccess");
        m.insert("mdz", "audio/x-mod");
        m.insert("mel", "text/x-vmel");
        m.insert("mht", "message/rfc822");
        m.insert("mhtml", "message/rfc822");
        m.insert("mi", "application/x-mif");
        m.insert("mil", "image/x-cals");
        m.insert("mio", "audio/x-mio");
        m.insert("mmf", "application/x-skt-lbs");
        m.insert("mng", "video/x-mng");
        m.insert("mny", "application/x-msmoney");
        m.insert("moc", "application/x-mocha");
        m.insert("mocha", "application/x-mocha");
        m.insert("mod", "audio/x-mod");
        m.insert("mof", "application/x-yumekara");
        m.insert("mol", "chemical/x-mdl-molfile");
        m.insert("mop", "chemical/x-mopac-input");
        m.insert("mpa", "video/mpeg");
        m.insert("mpc", "application/vnd.mpohun.certificate");
        m.insert("mpg4", "video/mp4");
        m.insert("mpn", "application/vnd.mophun.application");
        m.insert("mpp", "application/vnd.ms-project");
        m.insert("mps", "application/x-mapserver");
        m.insert("mpv2", "video/mpeg");
        m.insert("mrl", "text/x-mrml");
        m.insert("mrm", "application/x-mrm");
        m.insert("msg", "application/vnd.ms-outlook");
        m.insert("mts", "application/metastream");
        m.insert("mtx", "application/metastream");
        m.insert("mtz", "application/metastream");
        m.insert("mvb", "application/x-msmediaview");
        m.insert("mzv", "application/metastream");
        m.insert("nar", "application/zip");
        m.insert("nbmp", "image/nbmp");
        m.insert("ndb", "x-lml/x-ndb");
        m.insert("ndwn", "application/ndwn");
        m.insert("nif", "application/x-nif");
        m.insert("nmz", "application/x-scream");
        m.insert("nokia-op-logo", "image/vnd.nok-oplogo-color");
        m.insert("npx", "application/x-netfpx");
        m.insert("nsnd", "audio/nsnd");
        m.insert("nva", "application/x-neva1");
        m.insert("nws", "message/rfc822");
        m.insert("oom", "application/x-AtlasMate-Plugin");
        m.insert("p10", "application/pkcs10");
        m.insert("p12", "application/x-pkcs12");
        m.insert("p7b", "application/x-pkcs7-certificates");
        m.insert("p7c", "application/x-pkcs7-mime");
        m.insert("p7m", "application/x-pkcs7-mime");
        m.insert("p7r", "application/x-pkcs7-certreqresp");
        m.insert("p7s", "application/x-pkcs7-signature");
        m.insert("pac", "audio/x-pac");
        m.insert("pae", "audio/x-epac");
        m.insert("pan", "application/x-pan");
        m.insert("pcx", "image/x-pcx");
        m.insert("pda", "image/x-pda");
        m.insert("pfr", "application/font-tdpfr");
        m.insert("pfx", "application/x-pkcs12");
        m.insert("pko", "application/ynd.ms-pkipko");
        m.insert("pm", "application/x-perl");
        m.insert("pma", "application/x-perfmon");
        m.insert("pmc", "application/x-perfmon");
        m.insert("pmd", "application/x-pmd");
        m.insert("pml", "application/x-perfmon");
        m.insert("pmr", "application/x-perfmon");
        m.insert("pmw", "application/x-perfmon");
        m.insert("pnz", "image/png");
        m.insert("pot);", "application/vnd.ms-powerpoint");
        m.insert("pps", "application/vnd.ms-powerpoint");
        m.insert("pqf", "application/x-cprplayer");
        m.insert("pqi", "application/cprplayer");
        m.insert("prc", "application/x-prc");
        m.insert("prf", "application/pics-rules");
        m.insert("prop", "text/plain");
        m.insert("proxy", "application/x-ns-proxy-autoconfig");
        m.insert("ptlk", "application/listenup");
        m.insert("pub", "application/x-mspublisher");
        m.insert("pvx", "video/x-pv-pvx");
        m.insert("qcp", "audio/vnd.qcelp");
        m.insert("r3t", "text/vnd.rn-realtext3d");
        m.insert("rar", "application/octet-stream");
        m.insert("rc", "text/plain");
        m.insert("rf", "image/vnd.rn-realflash");
        m.insert("rlf", "application/x-richlink");
        m.insert("rmf", "audio/x-rmf");
        m.insert("rmi", "audio/mid");
        m.insert("rmm", "audio/x-pn-realaudio");
        m.insert("rmvb", "audio/x-pn-realaudio");
        m.insert("rnx", "application/vnd.rn-realplayer");
        m.insert("rp", "image/vnd.rn-realpix");
        m.insert("rt", "text/vnd.rn-realtext");
        m.insert("rte", "x-lml/x-gps");
        m.insert("rtg", "application/metastream");
        m.insert("rv", "video/vnd.rn-realvideo");
        m.insert("rwc", "application/x-rogerwilco");
        m.insert("s3m", "audio/x-mod");
        m.insert("s3z", "audio/x-mod");
        m.insert("sca", "application/x-supercard");
        m.insert("scd", "application/x-msschedule");
        m.insert("sct", "text/scriptlet");
        m.insert("sdf", "application/e-score");
        m.insert("sea", "application/x-stuffit");
        m.insert("setpay", "application/set-payment-initiation");
        m.insert("setreg", "application/set-registration-initiation");
        m.insert("shtml", "text/html");
        m.insert("shtm", "text/html");
        m.insert("shw", "application/presentations");
        m.insert("si6", "image/si6");
        m.insert("si7", "image/vnd.stiwap.sis");
        m.insert("si9", "image/vnd.lgtwap.sis");
        m.insert("slc", "application/x-salsa");
        m.insert("smd", "audio/x-smd");
        m.insert("smp", "application/studiom");
        m.insert("smz", "audio/x-smd");
        m.insert("spc", "application/x-pkcs7-certificates");
        m.insert("spr", "application/x-sprite");
        m.insert("sprite", "application/x-sprite");
        m.insert("sdp", "application/sdp");
        m.insert("spt", "application/x-spt");
        m.insert("sst", "application/vnd.ms-pkicertstore");
        m.insert("stk", "application/hyperstudio");
        m.insert("stl", "application/vnd.ms-pkistl");
        m.insert("stm", "text/html");
        m.insert("svf", "image/vnd");
        m.insert("svh", "image/svh");
        m.insert("svr", "x-world/x-svr");
        m.insert("swfl", "application/x-shockwave-flash");
        m.insert("tad", "application/octet-stream");
        m.insert("talk", "text/x-speech");
        m.insert("taz", "application/x-tar");
        m.insert("tbp", "application/x-timbuktu");
        m.insert("tbt", "application/x-timbuktu");
        m.insert("tgz", "application/x-compressed");
        m.insert("thm", "application/vnd.eri.thm");
        m.insert("tki", "application/x-tkined");
        m.insert("tkined", "application/x-tkined");
        m.insert("toc", "application/toc");
        m.insert("toy", "image/toy");
        m.insert("trk", "x-lml/x-gps");
        m.insert("trm", "application/x-msterminal");
        m.insert("tsi", "audio/tsplayer");
        m.insert("tsp", "application/dsptype");
        m.insert("ttf", "application/octet-stream");
        m.insert("ttz", "application/t-time");
        m.insert("uls", "text/iuls");
        m.insert("ult", "audio/x-mod");
        m.insert("uu", "application/x-uuencode");
        m.insert("uue", "application/x-uuencode");
        m.insert("vcf", "text/x-vcard");
        m.insert("vdo", "video/vdo");
        m.insert("vib", "audio/vib");
        m.insert("viv", "video/vivo");
        m.insert("vivo", "video/vivo");
        m.insert("vmd", "application/vocaltec-media-desc");
        m.insert("vmf", "application/vocaltec-media-file");
        m.insert("vmi", "application/x-dreamcast-vms-info");
        m.insert("vms", "application/x-dreamcast-vms");
        m.insert("vox", "audio/voxware");
        m.insert("vqe", "audio/x-twinvq-plugin");
        m.insert("vqf", "audio/x-twinvq");
        m.insert("vql", "audio/x-twinvq");
        m.insert("vre", "x-world/x-vream");
        m.insert("vrt", "x-world/x-vrt");
        m.insert("vrw", "x-world/x-vream");
        m.insert("vts", "workbook/formulaone");
        m.insert("wcm", "application/vnd.ms-works");
        m.insert("wdb", "application/vnd.ms-works");
        m.insert("web", "application/vnd.xara");
        m.insert("wi", "image/wavelet");
        m.insert("wis", "application/x-InstallShield");
        m.insert("wks", "application/vnd.ms-works");
        m.insert("wmd", "application/x-ms-wmd");
        m.insert("wmf", "application/x-msmetafile");
        m.insert("wmlscript", "text/vnd.wap.wmlscript");
        m.insert("wmz", "application/x-ms-wmz");
        m.insert("wpng", "image/x-up-wpng");
        m.insert("wps", "application/vnd.ms-works");
        m.insert("wpt", "x-lml/x-gps");
        m.insert("wri", "application/x-mswrite");
        m.insert("wrz", "x-world/x-vrml");
        m.insert("ws", "text/vnd.wap.wmlscript");
        m.insert("wsc", "application/vnd.wap.wmlscriptc");
        m.insert("wv", "video/wavelet");
        m.insert("wxl", "application/x-wxl");
        m.insert("x-gzip", "application/x-gzip");
        m.insert("xaf", "x-world/x-vrml");
        m.insert("xar", "application/vnd.xara");
        m.insert("xdm", "application/x-xdma");
        m.insert("xdma", "application/x-xdma");
        m.insert("xdw", "application/vnd.fujixerox.docuworks");
        m.insert("xhtm", "application/xhtml+xml");
        m.insert("xla", "application/vnd.ms-excel");
        m.insert("xlc", "application/vnd.ms-excel");
        m.insert("xll", "application/x-excel");
        m.insert("xlm", "application/vnd.ms-excel");
        m.insert("xlt", "application/vnd.ms-excel");
        m.insert("xlw", "application/vnd.ms-excel");
        m.insert("xm", "audio/x-mod");
        m.insert("xmz", "audio/x-mod");
        m.insert("xof", "x-world/x-vrml");
        m.insert("xpi", "application/x-xpinstall");
        m.insert("xsit", "text/xml");
        m.insert("yz1", "application/x-yz1");
        m.insert("z", "application/x-compress");
        m.insert("zac", "application/x-zaurus-zac");
        m.insert("json", "application/json");
        m
    };
}

#[allow(unused)]
pub(crate) fn type_by_extension(file_path: &str) -> String {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    let typ = match from_ext(&ext).first() {
        Some(mime) => {
            let typ_str = mime.essence_str().to_string();
            if typ_str.starts_with("text/") && typ_str.contains("charset=") {
                remove_charset_in_mime_type(&typ_str)
            } else {
                typ_str
            }
        }
        None => EXT_TO_MIME_TYPE
            .get(ext.as_str())
            .cloned()
            .unwrap_or("")
            .to_string(),
    };
    typ
}

fn remove_charset_in_mime_type(typ: &str) -> String {
    let re = Regex::new(r";\s*charset=[^;]+").expect("Invalid regex");
    re.replace_all(typ, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_by_extension() {
        // Both have
        assert_eq!(type_by_extension("a.jpg"), "image/jpeg");

        // Only in mime_guess
        assert_eq!(type_by_extension("a.z4"), "application/x-zmachine");

        // Only in EXT_TO_MIME_TYPE
        assert_eq!(type_by_extension("a.lcr"), "application/x-digitalloca");
    }

    #[test]
    fn test_remove_charset_in_mime_type() {
        // Test with a mime type that has a charset
        assert_eq!(
            remove_charset_in_mime_type("text/html; charset=utf-8"),
            "text/html"
        );

        // Test with a mime type that has no charset
        assert_eq!(
            remove_charset_in_mime_type("application/json"),
            "application/json"
        );

        // Test with an empty mime type
        assert_eq!(remove_charset_in_mime_type(""), "");
    }
}
