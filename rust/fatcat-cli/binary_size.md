
## Binary Size

As of 2020-05-24, in early development, the relative binary sizes are:

    121 MB      default debug build
    12 MB       default release build
    8.2 MB      release build w/ LTO
    6.6 MB      release build w/ LTO, striped

After some small changes:

    5.9 MB      release build w/ LTO, size optimization, other flags
    4.1 MB      release build w/ LTO, size optimization, other flags, striped

Replacing reqwest with minreq:

    6.3 MB      release build w/ LTO, size optimization, other flags
    4.1 MB      release build w/ LTO, size optimization, other flags, striped

    (so, not worth it, at least while using fatcat_openapi with hyper+tokio)

Note that release builds with LTO take *quite* a long time (many minutes). We
probably don't want that to be the defualt for `fatcatd` builds.

    cargo bloat --release --crates

     File  .text      Size Crate
    12.2%  21.4% 1021.5KiB fatcat_cli
     7.1%  12.5%  596.7KiB fatcat_openapi
     6.3%  11.1%  529.6KiB reqwest
     6.2%  10.9%  518.5KiB std
     3.5%   6.1%  290.3KiB clap
     2.5%   4.3%  205.9KiB regex
     2.4%   4.2%  198.7KiB regex_syntax
     2.1%   3.6%  172.8KiB h2
     1.9%   3.4%  162.7KiB hyper
     1.8%   3.1%  149.9KiB futures
     1.4%   2.4%  116.9KiB serde_json
     1.3%   2.3%  111.2KiB macaroon
     1.0%   1.8%   85.3KiB unicode_normalization
     0.7%   1.3%   62.4KiB http
     0.6%   1.0%   50.1KiB serde
     0.6%   1.0%   47.5KiB url
     0.5%   0.9%   41.9KiB [Unknown]
     0.4%   0.8%   36.5KiB tokio_reactor
     0.4%   0.7%   31.8KiB env_logger
     0.3%   0.6%   26.6KiB chrono
     3.4%   5.9%  283.3KiB And 57 more crates. Use -n N to show more.
    57.2% 100.0%    4.7MiB .text section size, the file size is 8.2MiB


    bnewbold@orithena$ cargo bloat --release
        Finished release [optimized] target(s) in 0.27s
        Analyzing target/release/fatcat-cli

     File  .text    Size                 Crate Name
     0.4%   1.0% 53.2KiB                 regex <regex::exec::ExecNoSync as regex::re_trait::RegularExpression>::capture...
     0.4%   0.8% 44.1KiB          regex_syntax regex_syntax::ast::parse::ParserI<P>::parse_with_comments
     0.3%   0.7% 36.8KiB unicode_normalization unicode_normalization::tables::compatibility_fully_decomposed
     0.3%   0.6% 30.3KiB unicode_normalization unicode_normalization::tables::canonical_fully_decomposed
     0.2%   0.5% 25.2KiB         data_encoding data_encoding::Encoding::decode_mut
     0.2%   0.5% 24.0KiB       fatcat_openapi? <fatcat_openapi::models::_IMPL_DESERIALIZE_FOR_ReleaseEntity::<impl serd...
     0.2%   0.5% 23.5KiB                  clap clap::app::parser::Parser::get_matches_with
     0.2%   0.4% 21.7KiB                  clap clap::app::validator::Validator::validate
     0.2%   0.4% 20.6KiB                  http http::header::name::parse_hdr
     0.2%   0.4% 19.5KiB            fatcat_cli fatcat_cli::Specifier::get_from_api
     0.1%   0.3% 16.4KiB            fatcat_cli <&mut serde_json::de::Deserializer<R> as serde::de::Deserializer>::deser...
     0.1%   0.3% 16.4KiB            fatcat_cli <&mut serde_json::de::Deserializer<R> as serde::de::Deserializer>::deser...
     0.1%   0.3% 16.2KiB            fatcat_cli <&mut serde_json::de::Deserializer<R> as serde::de::Deserializer>::deser...
     0.1%   0.3% 16.1KiB            fatcat_cli fatcat_cli::run
     0.1%   0.3% 15.2KiB            fatcat_cli <&mut serde_json::de::Deserializer<R> as serde::de::Deserializer>::deser...
     0.1%   0.3% 14.3KiB           serde_json? <&mut serde_json::de::Deserializer<R> as serde::de::Deserializer>::deser...
     0.1%   0.3% 14.2KiB            fatcat_cli <&mut serde_json::de::Deserializer<R> as serde::de::Deserializer>::deser...
     0.1%   0.3% 14.0KiB                 regex regex::exec::ExecBuilder::build
     0.1%   0.3% 13.8KiB unicode_normalization unicode_normalization::tables::composition_table
     0.1%   0.3% 13.6KiB            fatcat_cli <&mut serde_json::de::Deserializer<R> as serde::de::Deserializer>::deser...
    38.6%  89.5%  4.5MiB                       And 13832 smaller methods. Use -n N to show more.
    43.1% 100.0%  5.1MiB                       .text section size, the file size is 11.8MiB

Low hanging fruit includes:

- reviewing features for reqwest, clap, regex, fatcat_openapi
- replace reqwest with something smaller
- use `ansi-term` (already part of clap)
- consider removing fancy clap features? meh
- look at graph; probably duplicate versions of things

