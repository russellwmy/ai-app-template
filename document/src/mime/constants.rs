pub(crate) const SUPPORTED_MIME_TYPES: &[&str] = &[
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.oasis.opendocument.text",
    "application/pdf",
];

// Reference https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
pub(crate) const MIME_TYPE_DATA: &str = r#"[
    {
        "extension": ".aac",
        "description": "AAC audio",
        "mime_type": "audio/aac"
    },
    {
        "extension": ".abw",
        "description": "[AbiWord](https://en.wikipedia.org/wiki/AbiWord) document",
        "mime_type": "application/x-abiword"
    },
    {
        "extension": ".arc",
        "description": "Archive document (multiple files embedded)",
        "mime_type": "application/x-freearc"
    },
    {
        "extension": ".avif",
        "description": "AVIF image",
        "mime_type": "image/avif"
    },
    {
        "extension": ".avi",
        "description": "AVI: Audio Video Interleave",
        "mime_type": "video/x-msvideo"
    },
    {
        "extension": ".azw",
        "description": "Amazon Kindle eBook format",
        "mime_type": "application/vnd.amazon.ebook"
    },
    {
        "extension": ".bin",
        "description": "Any kind of binary data",
        "mime_type": "application/octet-stream"
    },
    {
        "extension": ".bmp",
        "description": "Windows OS/2 Bitmap Graphics",
        "mime_type": "image/bmp"
    },
    {
        "extension": ".bz",
        "description": "BZip archive",
        "mime_type": "application/x-bzip"
    },
    {
        "extension": ".bz2",
        "description": "BZip2 archive",
        "mime_type": "application/x-bzip2"
    },
    {
        "extension": ".cda",
        "description": "CD audio",
        "mime_type": "application/x-cdf"
    },
    {
        "extension": ".csh",
        "description": "C-Shell script",
        "mime_type": "application/x-csh"
    },
    {
        "extension": ".css",
        "description": "Cascading Style Sheets (CSS)",
        "mime_type": "text/css"
    },
    {
        "extension": ".csv",
        "description": "Comma-separated values (CSV)",
        "mime_type": "text/csv"
    },
    {
        "extension": ".doc",
        "description": "Microsoft Word",
        "mime_type": "application/msword"
    },
    {
        "extension": ".docx",
        "description": "Microsoft Word (OpenXML)",
        "mime_type": "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    },
    {
        "extension": ".eot",
        "description": "MS Embedded OpenType fonts",
        "mime_type": "application/vnd.ms-fontobject"
    },
    {
        "extension": ".epub",
        "description": "Electronic publication (EPUB)",
        "mime_type": "application/epub+zip"
    },
    {
        "extension": ".gz",
        "description": "GZip Compressed Archive",
        "mime_type": "application/gzip"
    },
    {
        "extension": ".gif",
        "description": "Graphics Interchange Format (GIF)",
        "mime_type": "image/gif"
    },
    {
        "extension": ".htm, .html",
        "description": "HyperText Markup Language (HTML)",
        "mime_type": "text/html"
    },
    {
        "extension": ".ico",
        "description": "Icon format",
        "mime_type": "image/vnd.microsoft.icon"
    },
    {
        "extension": ".ics",
        "description": "iCalendar format",
        "mime_type": "text/calendar"
    },
    {
        "extension": ".jar",
        "description": "Java Archive (JAR)",
        "mime_type": "application/java-archive"
    },
    {
        "extension": ".jpeg, .jpg",
        "description": "JPEG images",
        "mime_type": "image/jpeg"
    },
    {
        "extension": ".js",
        "description": "JavaScript",
        "mime_type": "text/javascript"
    },
    {
        "extension": ".json",
        "description": "JSON format",
        "mime_type": "application/json"
    },
    {
        "extension": ".jsonld",
        "description": "JSON-LD format",
        "mime_type": "application/ld+json"
    },
    {
        "extension": ".mid",
        "description": "Musical Instrument Digital Interface (MIDI)",
        "mime_type": "audio/midi"
    },
    {
        "extension": ".midi",
        "description": "Musical Instrument Digital Interface (MIDI)",
        "mime_type": "audio/x-midi"
    },
    {
        "extension": ".mjs",
        "description": "JavaScript module",
        "mime_type": "text/javascript"
    },
    {
        "extension": ".mp3",
        "description": "MP3 audio",
        "mime_type": "audio/mpeg"
    },
    {
        "extension": ".mp4",
        "description": "MP4 video",
        "mime_type": "video/mp4"
    },
    {
        "extension": ".mpeg",
        "description": "MPEG Video",
        "mime_type": "video/mpeg"
    },
    {
        "extension": ".mpkg",
        "description": "Apple Installer Package",
        "mime_type": "application/vnd.apple.installer+xml"
    },
    {
        "extension": ".odp",
        "description": "OpenDocument presentation document",
        "mime_type": "application/vnd.oasis.opendocument.presentation"
    },
    {
        "extension": ".ods",
        "description": "OpenDocument spreadsheet document",
        "mime_type": "application/vnd.oasis.opendocument.spreadsheet"
    },
    {
        "extension": ".odt",
        "description": "OpenDocument text document",
        "mime_type": "application/vnd.oasis.opendocument.text"
    },
    {
        "extension": ".oga",
        "description": "OGG audio",
        "mime_type": "audio/ogg"
    },
    {
        "extension": ".ogv",
        "description": "OGG video",
        "mime_type": "video/ogg"
    },
    {
        "extension": ".ogx",
        "description": "OGG",
        "mime_type": "application/ogg"
    },
    {
        "extension": ".opus",
        "description": "Opus audio",
        "mime_type": "audio/opus"
    },
    {
        "extension": ".otf",
        "description": "OpenType font",
        "mime_type": "font/otf"
    },
    {
        "extension": ".png",
        "description": "Portable Network Graphics",
        "mime_type": "image/png"
    },
    {
        "extension": ".pdf",
        "description": "Adobe [Portable Document Format](https://www.adobe.com/acrobat/about-adobe-pdf.html) (PDF)",
        "mime_type": "application/pdf"
    },
    {
        "extension": ".php",
        "description": "Hypertext Preprocessor (**Personal Home Page**)",
        "mime_type": "application/x-httpd-php"
    },
    {
        "extension": ".ppt",
        "description": "Microsoft PowerPoint",
        "mime_type": "application/vnd.ms-powerpoint"
    },
    {
        "extension": ".pptx",
        "description": "Microsoft PowerPoint (OpenXML)",
        "mime_type": "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    },
    {
        "extension": ".rar",
        "description": "RAR archive",
        "mime_type": "application/vnd.rar"
    },
    {
        "extension": ".rtf",
        "description": "Rich Text Format (RTF)",
        "mime_type": "application/rtf"
    },
    {
        "extension": ".sh",
        "description": "Bourne shell script",
        "mime_type": "application/x-sh"
    },
    {
        "extension": ".svg",
        "description": "Scalable Vector Graphics (SVG)",
        "mime_type": "image/svg+xml"
    },
    {
        "extension": ".tar",
        "description": "Tape Archive (TAR)",
        "mime_type": "application/x-tar"
    },
    {
        "extension": ".tif, .tiff",
        "description": "Tagged Image File Format (TIFF)",
        "mime_type": "image/tiff"
    },
    {
        "extension": ".ts",
        "description": "MPEG transport stream",
        "mime_type": "video/mp2t"
    },
    {
        "extension": ".ttf",
        "description": "TrueType Font",
        "mime_type": "font/ttf"
    },
    {
        "extension": ".txt",
        "description": "Text, (generally ASCII or ISO 8859-_n_)",
        "mime_type": "text/plain"
    },
    {
        "extension": ".vsd",
        "description": "Microsoft Visio",
        "mime_type": "application/vnd.visio"
    },
    {
        "extension": ".wav",
        "description": "Waveform Audio Format",
        "mime_type": "audio/wav"
    },
    {
        "extension": ".weba",
        "description": "WEBM audio",
        "mime_type": "audio/webm"
    },
    {
        "extension": ".webm",
        "description": "WEBM video",
        "mime_type": "video/webm"
    },
    {
        "extension": ".webp",
        "description": "WEBP image",
        "mime_type": "image/webp"
    },
    {
        "extension": ".woff",
        "description": "Web Open Font Format (WOFF)",
        "mime_type": "font/woff"
    },
    {
        "extension": ".woff2",
        "description": "Web Open Font Format (WOFF)",
        "mime_type": "font/woff2"
    },
    {
        "extension": ".xhtml",
        "description": "XHTML",
        "mime_type": "application/xhtml+xml"
    },
    {
        "extension": ".xls",
        "description": "Microsoft Excel",
        "mime_type": "application/vnd.ms-excel"
    },
    {
        "extension": ".xlsx",
        "description": "Microsoft Excel (OpenXML)",
        "mime_type": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    },
    {
        "extension": ".xml",
        "description": "XML",
        "mime_type": "application/xml"
    },
    {
        "extension": ".xul",
        "description": "XUL",
        "mime_type": "application/vnd.mozilla.xul+xml"
    },
    {
        "extension": ".zip",
        "description": "ZIP archive",
        "mime_type": "application/zip"
    },
    {
        "extension": ".3gp",
        "description": "[3GPP](https://en.wikipedia.org/wiki/3GP_and_3G2) audio container",
        "mime_type": "audio/3gpp"
    },
    {
        "extension": ".3gp",
        "description": "[3GPP](https://en.wikipedia.org/wiki/3GP_and_3G2) video container",
        "mime_type": "video/3gpp"
    },
    {
        "extension": ".3g2",
        "description": "[3GPP2](https://en.wikipedia.org/wiki/3GP_and_3G2) audio container",
        "mime_type": "audio/3gpp2"
    },
    {
        "extension": ".3g2",
        "description": "[3GPP2](https://en.wikipedia.org/wiki/3GP_and_3G2) video container",
        "mime_type": "video/3gpp2"
    },
    {
        "extension": ".7z",
        "description": "[7-zip](https://en.wikipedia.org/wiki/7-Zip) archive",
        "mime_type": "application/x-7z-compressed"
    },
]"#;
