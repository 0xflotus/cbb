pub mod util;

#[test]
fn test() {
    const RESULTS: [&str; 3001] = [
        "0000", "000+", "00+-", "00+0", "00++", "0+--", "0+-0", "0+-+", "0+0-", "0+00", "0+0+",
        "0++-", "0++0", "0+++", "+---", "+--0", "+--+", "+-0-", "+-00", "+-0+", "+-+-", "+-+0",
        "+-++", "+0--", "+0-0", "+0-+", "+00-", "+000", "+00+", "+0+-", "+0+0", "+0++", "++--",
        "++-0", "++-+", "++0-", "++00", "++0+", "+++-", "+++0", "++++", "+----", "+---0", "+---+",
        "+--0-", "+--00", "+--0+", "+--+-", "+--+0", "+--++", "+-0--", "+-0-0", "+-0-+", "+-00-",
        "+-000", "+-00+", "+-0+-", "+-0+0", "+-0++", "+-+--", "+-+-0", "+-+-+", "+-+0-", "+-+00",
        "+-+0+", "+-++-", "+-++0", "+-+++", "+0---", "+0--0", "+0--+", "+0-0-", "+0-00", "+0-0+",
        "+0-+-", "+0-+0", "+0-++", "+00--", "+00-0", "+00-+", "+000-", "+0000", "+000+", "+00+-",
        "+00+0", "+00++", "+0+--", "+0+-0", "+0+-+", "+0+0-", "+0+00", "+0+0+", "+0++-", "+0++0",
        "+0+++", "++---", "++--0", "++--+", "++-0-", "++-00", "++-0+", "++-+-", "++-+0", "++-++",
        "++0--", "++0-0", "++0-+", "++00-", "++000", "++00+", "++0+-", "++0+0", "++0++", "+++--",
        "+++-0", "+++-+", "+++0-", "+++00", "+++0+", "++++-", "++++0", "+++++", "+-----", "+----0",
        "+----+", "+---0-", "+---00", "+---0+", "+---+-", "+---+0", "+---++", "+--0--", "+--0-0",
        "+--0-+", "+--00-", "+--000", "+--00+", "+--0+-", "+--0+0", "+--0++", "+--+--", "+--+-0",
        "+--+-+", "+--+0-", "+--+00", "+--+0+", "+--++-", "+--++0", "+--+++", "+-0---", "+-0--0",
        "+-0--+", "+-0-0-", "+-0-00", "+-0-0+", "+-0-+-", "+-0-+0", "+-0-++", "+-00--", "+-00-0",
        "+-00-+", "+-000-", "+-0000", "+-000+", "+-00+-", "+-00+0", "+-00++", "+-0+--", "+-0+-0",
        "+-0+-+", "+-0+0-", "+-0+00", "+-0+0+", "+-0++-", "+-0++0", "+-0+++", "+-+---", "+-+--0",
        "+-+--+", "+-+-0-", "+-+-00", "+-+-0+", "+-+-+-", "+-+-+0", "+-+-++", "+-+0--", "+-+0-0",
        "+-+0-+", "+-+00-", "+-+000", "+-+00+", "+-+0+-", "+-+0+0", "+-+0++", "+-++--", "+-++-0",
        "+-++-+", "+-++0-", "+-++00", "+-++0+", "+-+++-", "+-+++0", "+-++++", "+0----", "+0---0",
        "+0---+", "+0--0-", "+0--00", "+0--0+", "+0--+-", "+0--+0", "+0--++", "+0-0--", "+0-0-0",
        "+0-0-+", "+0-00-", "+0-000", "+0-00+", "+0-0+-", "+0-0+0", "+0-0++", "+0-+--", "+0-+-0",
        "+0-+-+", "+0-+0-", "+0-+00", "+0-+0+", "+0-++-", "+0-++0", "+0-+++", "+00---", "+00--0",
        "+00--+", "+00-0-", "+00-00", "+00-0+", "+00-+-", "+00-+0", "+00-++", "+000--", "+000-0",
        "+000-+", "+0000-", "+00000", "+0000+", "+000+-", "+000+0", "+000++", "+00+--", "+00+-0",
        "+00+-+", "+00+0-", "+00+00", "+00+0+", "+00++-", "+00++0", "+00+++", "+0+---", "+0+--0",
        "+0+--+", "+0+-0-", "+0+-00", "+0+-0+", "+0+-+-", "+0+-+0", "+0+-++", "+0+0--", "+0+0-0",
        "+0+0-+", "+0+00-", "+0+000", "+0+00+", "+0+0+-", "+0+0+0", "+0+0++", "+0++--", "+0++-0",
        "+0++-+", "+0++0-", "+0++00", "+0++0+", "+0+++-", "+0+++0", "+0++++", "++----", "++---0",
        "++---+", "++--0-", "++--00", "++--0+", "++--+-", "++--+0", "++--++", "++-0--", "++-0-0",
        "++-0-+", "++-00-", "++-000", "++-00+", "++-0+-", "++-0+0", "++-0++", "++-+--", "++-+-0",
        "++-+-+", "++-+0-", "++-+00", "++-+0+", "++-++-", "++-++0", "++-+++", "++0---", "++0--0",
        "++0--+", "++0-0-", "++0-00", "++0-0+", "++0-+-", "++0-+0", "++0-++", "++00--", "++00-0",
        "++00-+", "++000-", "++0000", "++000+", "++00+-", "++00+0", "++00++", "++0+--", "++0+-0",
        "++0+-+", "++0+0-", "++0+00", "++0+0+", "++0++-", "++0++0", "++0+++", "+++---", "+++--0",
        "+++--+", "+++-0-", "+++-00", "+++-0+", "+++-+-", "+++-+0", "+++-++", "+++0--", "+++0-0",
        "+++0-+", "+++00-", "+++000", "+++00+", "+++0+-", "+++0+0", "+++0++", "++++--", "++++-0",
        "++++-+", "++++0-", "++++00", "++++0+", "+++++-", "+++++0", "++++++", "+------", "+-----0",
        "+-----+", "+----0-", "+----00", "+----0+", "+----+-", "+----+0", "+----++", "+---0--",
        "+---0-0", "+---0-+", "+---00-", "+---000", "+---00+", "+---0+-", "+---0+0", "+---0++",
        "+---+--", "+---+-0", "+---+-+", "+---+0-", "+---+00", "+---+0+", "+---++-", "+---++0",
        "+---+++", "+--0---", "+--0--0", "+--0--+", "+--0-0-", "+--0-00", "+--0-0+", "+--0-+-",
        "+--0-+0", "+--0-++", "+--00--", "+--00-0", "+--00-+", "+--000-", "+--0000", "+--000+",
        "+--00+-", "+--00+0", "+--00++", "+--0+--", "+--0+-0", "+--0+-+", "+--0+0-", "+--0+00",
        "+--0+0+", "+--0++-", "+--0++0", "+--0+++", "+--+---", "+--+--0", "+--+--+", "+--+-0-",
        "+--+-00", "+--+-0+", "+--+-+-", "+--+-+0", "+--+-++", "+--+0--", "+--+0-0", "+--+0-+",
        "+--+00-", "+--+000", "+--+00+", "+--+0+-", "+--+0+0", "+--+0++", "+--++--", "+--++-0",
        "+--++-+", "+--++0-", "+--++00", "+--++0+", "+--+++-", "+--+++0", "+--++++", "+-0----",
        "+-0---0", "+-0---+", "+-0--0-", "+-0--00", "+-0--0+", "+-0--+-", "+-0--+0", "+-0--++",
        "+-0-0--", "+-0-0-0", "+-0-0-+", "+-0-00-", "+-0-000", "+-0-00+", "+-0-0+-", "+-0-0+0",
        "+-0-0++", "+-0-+--", "+-0-+-0", "+-0-+-+", "+-0-+0-", "+-0-+00", "+-0-+0+", "+-0-++-",
        "+-0-++0", "+-0-+++", "+-00---", "+-00--0", "+-00--+", "+-00-0-", "+-00-00", "+-00-0+",
        "+-00-+-", "+-00-+0", "+-00-++", "+-000--", "+-000-0", "+-000-+", "+-0000-", "+-00000",
        "+-0000+", "+-000+-", "+-000+0", "+-000++", "+-00+--", "+-00+-0", "+-00+-+", "+-00+0-",
        "+-00+00", "+-00+0+", "+-00++-", "+-00++0", "+-00+++", "+-0+---", "+-0+--0", "+-0+--+",
        "+-0+-0-", "+-0+-00", "+-0+-0+", "+-0+-+-", "+-0+-+0", "+-0+-++", "+-0+0--", "+-0+0-0",
        "+-0+0-+", "+-0+00-", "+-0+000", "+-0+00+", "+-0+0+-", "+-0+0+0", "+-0+0++", "+-0++--",
        "+-0++-0", "+-0++-+", "+-0++0-", "+-0++00", "+-0++0+", "+-0+++-", "+-0+++0", "+-0++++",
        "+-+----", "+-+---0", "+-+---+", "+-+--0-", "+-+--00", "+-+--0+", "+-+--+-", "+-+--+0",
        "+-+--++", "+-+-0--", "+-+-0-0", "+-+-0-+", "+-+-00-", "+-+-000", "+-+-00+", "+-+-0+-",
        "+-+-0+0", "+-+-0++", "+-+-+--", "+-+-+-0", "+-+-+-+", "+-+-+0-", "+-+-+00", "+-+-+0+",
        "+-+-++-", "+-+-++0", "+-+-+++", "+-+0---", "+-+0--0", "+-+0--+", "+-+0-0-", "+-+0-00",
        "+-+0-0+", "+-+0-+-", "+-+0-+0", "+-+0-++", "+-+00--", "+-+00-0", "+-+00-+", "+-+000-",
        "+-+0000", "+-+000+", "+-+00+-", "+-+00+0", "+-+00++", "+-+0+--", "+-+0+-0", "+-+0+-+",
        "+-+0+0-", "+-+0+00", "+-+0+0+", "+-+0++-", "+-+0++0", "+-+0+++", "+-++---", "+-++--0",
        "+-++--+", "+-++-0-", "+-++-00", "+-++-0+", "+-++-+-", "+-++-+0", "+-++-++", "+-++0--",
        "+-++0-0", "+-++0-+", "+-++00-", "+-++000", "+-++00+", "+-++0+-", "+-++0+0", "+-++0++",
        "+-+++--", "+-+++-0", "+-+++-+", "+-+++0-", "+-+++00", "+-+++0+", "+-++++-", "+-++++0",
        "+-+++++", "+0-----", "+0----0", "+0----+", "+0---0-", "+0---00", "+0---0+", "+0---+-",
        "+0---+0", "+0---++", "+0--0--", "+0--0-0", "+0--0-+", "+0--00-", "+0--000", "+0--00+",
        "+0--0+-", "+0--0+0", "+0--0++", "+0--+--", "+0--+-0", "+0--+-+", "+0--+0-", "+0--+00",
        "+0--+0+", "+0--++-", "+0--++0", "+0--+++", "+0-0---", "+0-0--0", "+0-0--+", "+0-0-0-",
        "+0-0-00", "+0-0-0+", "+0-0-+-", "+0-0-+0", "+0-0-++", "+0-00--", "+0-00-0", "+0-00-+",
        "+0-000-", "+0-0000", "+0-000+", "+0-00+-", "+0-00+0", "+0-00++", "+0-0+--", "+0-0+-0",
        "+0-0+-+", "+0-0+0-", "+0-0+00", "+0-0+0+", "+0-0++-", "+0-0++0", "+0-0+++", "+0-+---",
        "+0-+--0", "+0-+--+", "+0-+-0-", "+0-+-00", "+0-+-0+", "+0-+-+-", "+0-+-+0", "+0-+-++",
        "+0-+0--", "+0-+0-0", "+0-+0-+", "+0-+00-", "+0-+000", "+0-+00+", "+0-+0+-", "+0-+0+0",
        "+0-+0++", "+0-++--", "+0-++-0", "+0-++-+", "+0-++0-", "+0-++00", "+0-++0+", "+0-+++-",
        "+0-+++0", "+0-++++", "+00----", "+00---0", "+00---+", "+00--0-", "+00--00", "+00--0+",
        "+00--+-", "+00--+0", "+00--++", "+00-0--", "+00-0-0", "+00-0-+", "+00-00-", "+00-000",
        "+00-00+", "+00-0+-", "+00-0+0", "+00-0++", "+00-+--", "+00-+-0", "+00-+-+", "+00-+0-",
        "+00-+00", "+00-+0+", "+00-++-", "+00-++0", "+00-+++", "+000---", "+000--0", "+000--+",
        "+000-0-", "+000-00", "+000-0+", "+000-+-", "+000-+0", "+000-++", "+0000--", "+0000-0",
        "+0000-+", "+00000-", "+000000", "+00000+", "+0000+-", "+0000+0", "+0000++", "+000+--",
        "+000+-0", "+000+-+", "+000+0-", "+000+00", "+000+0+", "+000++-", "+000++0", "+000+++",
        "+00+---", "+00+--0", "+00+--+", "+00+-0-", "+00+-00", "+00+-0+", "+00+-+-", "+00+-+0",
        "+00+-++", "+00+0--", "+00+0-0", "+00+0-+", "+00+00-", "+00+000", "+00+00+", "+00+0+-",
        "+00+0+0", "+00+0++", "+00++--", "+00++-0", "+00++-+", "+00++0-", "+00++00", "+00++0+",
        "+00+++-", "+00+++0", "+00++++", "+0+----", "+0+---0", "+0+---+", "+0+--0-", "+0+--00",
        "+0+--0+", "+0+--+-", "+0+--+0", "+0+--++", "+0+-0--", "+0+-0-0", "+0+-0-+", "+0+-00-",
        "+0+-000", "+0+-00+", "+0+-0+-", "+0+-0+0", "+0+-0++", "+0+-+--", "+0+-+-0", "+0+-+-+",
        "+0+-+0-", "+0+-+00", "+0+-+0+", "+0+-++-", "+0+-++0", "+0+-+++", "+0+0---", "+0+0--0",
        "+0+0--+", "+0+0-0-", "+0+0-00", "+0+0-0+", "+0+0-+-", "+0+0-+0", "+0+0-++", "+0+00--",
        "+0+00-0", "+0+00-+", "+0+000-", "+0+0000", "+0+000+", "+0+00+-", "+0+00+0", "+0+00++",
        "+0+0+--", "+0+0+-0", "+0+0+-+", "+0+0+0-", "+0+0+00", "+0+0+0+", "+0+0++-", "+0+0++0",
        "+0+0+++", "+0++---", "+0++--0", "+0++--+", "+0++-0-", "+0++-00", "+0++-0+", "+0++-+-",
        "+0++-+0", "+0++-++", "+0++0--", "+0++0-0", "+0++0-+", "+0++00-", "+0++000", "+0++00+",
        "+0++0+-", "+0++0+0", "+0++0++", "+0+++--", "+0+++-0", "+0+++-+", "+0+++0-", "+0+++00",
        "+0+++0+", "+0++++-", "+0++++0", "+0+++++", "++-----", "++----0", "++----+", "++---0-",
        "++---00", "++---0+", "++---+-", "++---+0", "++---++", "++--0--", "++--0-0", "++--0-+",
        "++--00-", "++--000", "++--00+", "++--0+-", "++--0+0", "++--0++", "++--+--", "++--+-0",
        "++--+-+", "++--+0-", "++--+00", "++--+0+", "++--++-", "++--++0", "++--+++", "++-0---",
        "++-0--0", "++-0--+", "++-0-0-", "++-0-00", "++-0-0+", "++-0-+-", "++-0-+0", "++-0-++",
        "++-00--", "++-00-0", "++-00-+", "++-000-", "++-0000", "++-000+", "++-00+-", "++-00+0",
        "++-00++", "++-0+--", "++-0+-0", "++-0+-+", "++-0+0-", "++-0+00", "++-0+0+", "++-0++-",
        "++-0++0", "++-0+++", "++-+---", "++-+--0", "++-+--+", "++-+-0-", "++-+-00", "++-+-0+",
        "++-+-+-", "++-+-+0", "++-+-++", "++-+0--", "++-+0-0", "++-+0-+", "++-+00-", "++-+000",
        "++-+00+", "++-+0+-", "++-+0+0", "++-+0++", "++-++--", "++-++-0", "++-++-+", "++-++0-",
        "++-++00", "++-++0+", "++-+++-", "++-+++0", "++-++++", "++0----", "++0---0", "++0---+",
        "++0--0-", "++0--00", "++0--0+", "++0--+-", "++0--+0", "++0--++", "++0-0--", "++0-0-0",
        "++0-0-+", "++0-00-", "++0-000", "++0-00+", "++0-0+-", "++0-0+0", "++0-0++", "++0-+--",
        "++0-+-0", "++0-+-+", "++0-+0-", "++0-+00", "++0-+0+", "++0-++-", "++0-++0", "++0-+++",
        "++00---", "++00--0", "++00--+", "++00-0-", "++00-00", "++00-0+", "++00-+-", "++00-+0",
        "++00-++", "++000--", "++000-0", "++000-+", "++0000-", "++00000", "++0000+", "++000+-",
        "++000+0", "++000++", "++00+--", "++00+-0", "++00+-+", "++00+0-", "++00+00", "++00+0+",
        "++00++-", "++00++0", "++00+++", "++0+---", "++0+--0", "++0+--+", "++0+-0-", "++0+-00",
        "++0+-0+", "++0+-+-", "++0+-+0", "++0+-++", "++0+0--", "++0+0-0", "++0+0-+", "++0+00-",
        "++0+000", "++0+00+", "++0+0+-", "++0+0+0", "++0+0++", "++0++--", "++0++-0", "++0++-+",
        "++0++0-", "++0++00", "++0++0+", "++0+++-", "++0+++0", "++0++++", "+++----", "+++---0",
        "+++---+", "+++--0-", "+++--00", "+++--0+", "+++--+-", "+++--+0", "+++--++", "+++-0--",
        "+++-0-0", "+++-0-+", "+++-00-", "+++-000", "+++-00+", "+++-0+-", "+++-0+0", "+++-0++",
        "+++-+--", "+++-+-0", "+++-+-+", "+++-+0-", "+++-+00", "+++-+0+", "+++-++-", "+++-++0",
        "+++-+++", "+++0---", "+++0--0", "+++0--+", "+++0-0-", "+++0-00", "+++0-0+", "+++0-+-",
        "+++0-+0", "+++0-++", "+++00--", "+++00-0", "+++00-+", "+++000-", "+++0000", "+++000+",
        "+++00+-", "+++00+0", "+++00++", "+++0+--", "+++0+-0", "+++0+-+", "+++0+0-", "+++0+00",
        "+++0+0+", "+++0++-", "+++0++0", "+++0+++", "++++---", "++++--0", "++++--+", "++++-0-",
        "++++-00", "++++-0+", "++++-+-", "++++-+0", "++++-++", "++++0--", "++++0-0", "++++0-+",
        "++++00-", "++++000", "++++00+", "++++0+-", "++++0+0", "++++0++", "+++++--", "+++++-0",
        "+++++-+", "+++++0-", "+++++00", "+++++0+", "++++++-", "++++++0", "+++++++", "+-------",
        "+------0", "+------+", "+-----0-", "+-----00", "+-----0+", "+-----+-", "+-----+0",
        "+-----++", "+----0--", "+----0-0", "+----0-+", "+----00-", "+----000", "+----00+",
        "+----0+-", "+----0+0", "+----0++", "+----+--", "+----+-0", "+----+-+", "+----+0-",
        "+----+00", "+----+0+", "+----++-", "+----++0", "+----+++", "+---0---", "+---0--0",
        "+---0--+", "+---0-0-", "+---0-00", "+---0-0+", "+---0-+-", "+---0-+0", "+---0-++",
        "+---00--", "+---00-0", "+---00-+", "+---000-", "+---0000", "+---000+", "+---00+-",
        "+---00+0", "+---00++", "+---0+--", "+---0+-0", "+---0+-+", "+---0+0-", "+---0+00",
        "+---0+0+", "+---0++-", "+---0++0", "+---0+++", "+---+---", "+---+--0", "+---+--+",
        "+---+-0-", "+---+-00", "+---+-0+", "+---+-+-", "+---+-+0", "+---+-++", "+---+0--",
        "+---+0-0", "+---+0-+", "+---+00-", "+---+000", "+---+00+", "+---+0+-", "+---+0+0",
        "+---+0++", "+---++--", "+---++-0", "+---++-+", "+---++0-", "+---++00", "+---++0+",
        "+---+++-", "+---+++0", "+---++++", "+--0----", "+--0---0", "+--0---+", "+--0--0-",
        "+--0--00", "+--0--0+", "+--0--+-", "+--0--+0", "+--0--++", "+--0-0--", "+--0-0-0",
        "+--0-0-+", "+--0-00-", "+--0-000", "+--0-00+", "+--0-0+-", "+--0-0+0", "+--0-0++",
        "+--0-+--", "+--0-+-0", "+--0-+-+", "+--0-+0-", "+--0-+00", "+--0-+0+", "+--0-++-",
        "+--0-++0", "+--0-+++", "+--00---", "+--00--0", "+--00--+", "+--00-0-", "+--00-00",
        "+--00-0+", "+--00-+-", "+--00-+0", "+--00-++", "+--000--", "+--000-0", "+--000-+",
        "+--0000-", "+--00000", "+--0000+", "+--000+-", "+--000+0", "+--000++", "+--00+--",
        "+--00+-0", "+--00+-+", "+--00+0-", "+--00+00", "+--00+0+", "+--00++-", "+--00++0",
        "+--00+++", "+--0+---", "+--0+--0", "+--0+--+", "+--0+-0-", "+--0+-00", "+--0+-0+",
        "+--0+-+-", "+--0+-+0", "+--0+-++", "+--0+0--", "+--0+0-0", "+--0+0-+", "+--0+00-",
        "+--0+000", "+--0+00+", "+--0+0+-", "+--0+0+0", "+--0+0++", "+--0++--", "+--0++-0",
        "+--0++-+", "+--0++0-", "+--0++00", "+--0++0+", "+--0+++-", "+--0+++0", "+--0++++",
        "+--+----", "+--+---0", "+--+---+", "+--+--0-", "+--+--00", "+--+--0+", "+--+--+-",
        "+--+--+0", "+--+--++", "+--+-0--", "+--+-0-0", "+--+-0-+", "+--+-00-", "+--+-000",
        "+--+-00+", "+--+-0+-", "+--+-0+0", "+--+-0++", "+--+-+--", "+--+-+-0", "+--+-+-+",
        "+--+-+0-", "+--+-+00", "+--+-+0+", "+--+-++-", "+--+-++0", "+--+-+++", "+--+0---",
        "+--+0--0", "+--+0--+", "+--+0-0-", "+--+0-00", "+--+0-0+", "+--+0-+-", "+--+0-+0",
        "+--+0-++", "+--+00--", "+--+00-0", "+--+00-+", "+--+000-", "+--+0000", "+--+000+",
        "+--+00+-", "+--+00+0", "+--+00++", "+--+0+--", "+--+0+-0", "+--+0+-+", "+--+0+0-",
        "+--+0+00", "+--+0+0+", "+--+0++-", "+--+0++0", "+--+0+++", "+--++---", "+--++--0",
        "+--++--+", "+--++-0-", "+--++-00", "+--++-0+", "+--++-+-", "+--++-+0", "+--++-++",
        "+--++0--", "+--++0-0", "+--++0-+", "+--++00-", "+--++000", "+--++00+", "+--++0+-",
        "+--++0+0", "+--++0++", "+--+++--", "+--+++-0", "+--+++-+", "+--+++0-", "+--+++00",
        "+--+++0+", "+--++++-", "+--++++0", "+--+++++", "+-0-----", "+-0----0", "+-0----+",
        "+-0---0-", "+-0---00", "+-0---0+", "+-0---+-", "+-0---+0", "+-0---++", "+-0--0--",
        "+-0--0-0", "+-0--0-+", "+-0--00-", "+-0--000", "+-0--00+", "+-0--0+-", "+-0--0+0",
        "+-0--0++", "+-0--+--", "+-0--+-0", "+-0--+-+", "+-0--+0-", "+-0--+00", "+-0--+0+",
        "+-0--++-", "+-0--++0", "+-0--+++", "+-0-0---", "+-0-0--0", "+-0-0--+", "+-0-0-0-",
        "+-0-0-00", "+-0-0-0+", "+-0-0-+-", "+-0-0-+0", "+-0-0-++", "+-0-00--", "+-0-00-0",
        "+-0-00-+", "+-0-000-", "+-0-0000", "+-0-000+", "+-0-00+-", "+-0-00+0", "+-0-00++",
        "+-0-0+--", "+-0-0+-0", "+-0-0+-+", "+-0-0+0-", "+-0-0+00", "+-0-0+0+", "+-0-0++-",
        "+-0-0++0", "+-0-0+++", "+-0-+---", "+-0-+--0", "+-0-+--+", "+-0-+-0-", "+-0-+-00",
        "+-0-+-0+", "+-0-+-+-", "+-0-+-+0", "+-0-+-++", "+-0-+0--", "+-0-+0-0", "+-0-+0-+",
        "+-0-+00-", "+-0-+000", "+-0-+00+", "+-0-+0+-", "+-0-+0+0", "+-0-+0++", "+-0-++--",
        "+-0-++-0", "+-0-++-+", "+-0-++0-", "+-0-++00", "+-0-++0+", "+-0-+++-", "+-0-+++0",
        "+-0-++++", "+-00----", "+-00---0", "+-00---+", "+-00--0-", "+-00--00", "+-00--0+",
        "+-00--+-", "+-00--+0", "+-00--++", "+-00-0--", "+-00-0-0", "+-00-0-+", "+-00-00-",
        "+-00-000", "+-00-00+", "+-00-0+-", "+-00-0+0", "+-00-0++", "+-00-+--", "+-00-+-0",
        "+-00-+-+", "+-00-+0-", "+-00-+00", "+-00-+0+", "+-00-++-", "+-00-++0", "+-00-+++",
        "+-000---", "+-000--0", "+-000--+", "+-000-0-", "+-000-00", "+-000-0+", "+-000-+-",
        "+-000-+0", "+-000-++", "+-0000--", "+-0000-0", "+-0000-+", "+-00000-", "+-000000",
        "+-00000+", "+-0000+-", "+-0000+0", "+-0000++", "+-000+--", "+-000+-0", "+-000+-+",
        "+-000+0-", "+-000+00", "+-000+0+", "+-000++-", "+-000++0", "+-000+++", "+-00+---",
        "+-00+--0", "+-00+--+", "+-00+-0-", "+-00+-00", "+-00+-0+", "+-00+-+-", "+-00+-+0",
        "+-00+-++", "+-00+0--", "+-00+0-0", "+-00+0-+", "+-00+00-", "+-00+000", "+-00+00+",
        "+-00+0+-", "+-00+0+0", "+-00+0++", "+-00++--", "+-00++-0", "+-00++-+", "+-00++0-",
        "+-00++00", "+-00++0+", "+-00+++-", "+-00+++0", "+-00++++", "+-0+----", "+-0+---0",
        "+-0+---+", "+-0+--0-", "+-0+--00", "+-0+--0+", "+-0+--+-", "+-0+--+0", "+-0+--++",
        "+-0+-0--", "+-0+-0-0", "+-0+-0-+", "+-0+-00-", "+-0+-000", "+-0+-00+", "+-0+-0+-",
        "+-0+-0+0", "+-0+-0++", "+-0+-+--", "+-0+-+-0", "+-0+-+-+", "+-0+-+0-", "+-0+-+00",
        "+-0+-+0+", "+-0+-++-", "+-0+-++0", "+-0+-+++", "+-0+0---", "+-0+0--0", "+-0+0--+",
        "+-0+0-0-", "+-0+0-00", "+-0+0-0+", "+-0+0-+-", "+-0+0-+0", "+-0+0-++", "+-0+00--",
        "+-0+00-0", "+-0+00-+", "+-0+000-", "+-0+0000", "+-0+000+", "+-0+00+-", "+-0+00+0",
        "+-0+00++", "+-0+0+--", "+-0+0+-0", "+-0+0+-+", "+-0+0+0-", "+-0+0+00", "+-0+0+0+",
        "+-0+0++-", "+-0+0++0", "+-0+0+++", "+-0++---", "+-0++--0", "+-0++--+", "+-0++-0-",
        "+-0++-00", "+-0++-0+", "+-0++-+-", "+-0++-+0", "+-0++-++", "+-0++0--", "+-0++0-0",
        "+-0++0-+", "+-0++00-", "+-0++000", "+-0++00+", "+-0++0+-", "+-0++0+0", "+-0++0++",
        "+-0+++--", "+-0+++-0", "+-0+++-+", "+-0+++0-", "+-0+++00", "+-0+++0+", "+-0++++-",
        "+-0++++0", "+-0+++++", "+-+-----", "+-+----0", "+-+----+", "+-+---0-", "+-+---00",
        "+-+---0+", "+-+---+-", "+-+---+0", "+-+---++", "+-+--0--", "+-+--0-0", "+-+--0-+",
        "+-+--00-", "+-+--000", "+-+--00+", "+-+--0+-", "+-+--0+0", "+-+--0++", "+-+--+--",
        "+-+--+-0", "+-+--+-+", "+-+--+0-", "+-+--+00", "+-+--+0+", "+-+--++-", "+-+--++0",
        "+-+--+++", "+-+-0---", "+-+-0--0", "+-+-0--+", "+-+-0-0-", "+-+-0-00", "+-+-0-0+",
        "+-+-0-+-", "+-+-0-+0", "+-+-0-++", "+-+-00--", "+-+-00-0", "+-+-00-+", "+-+-000-",
        "+-+-0000", "+-+-000+", "+-+-00+-", "+-+-00+0", "+-+-00++", "+-+-0+--", "+-+-0+-0",
        "+-+-0+-+", "+-+-0+0-", "+-+-0+00", "+-+-0+0+", "+-+-0++-", "+-+-0++0", "+-+-0+++",
        "+-+-+---", "+-+-+--0", "+-+-+--+", "+-+-+-0-", "+-+-+-00", "+-+-+-0+", "+-+-+-+-",
        "+-+-+-+0", "+-+-+-++", "+-+-+0--", "+-+-+0-0", "+-+-+0-+", "+-+-+00-", "+-+-+000",
        "+-+-+00+", "+-+-+0+-", "+-+-+0+0", "+-+-+0++", "+-+-++--", "+-+-++-0", "+-+-++-+",
        "+-+-++0-", "+-+-++00", "+-+-++0+", "+-+-+++-", "+-+-+++0", "+-+-++++", "+-+0----",
        "+-+0---0", "+-+0---+", "+-+0--0-", "+-+0--00", "+-+0--0+", "+-+0--+-", "+-+0--+0",
        "+-+0--++", "+-+0-0--", "+-+0-0-0", "+-+0-0-+", "+-+0-00-", "+-+0-000", "+-+0-00+",
        "+-+0-0+-", "+-+0-0+0", "+-+0-0++", "+-+0-+--", "+-+0-+-0", "+-+0-+-+", "+-+0-+0-",
        "+-+0-+00", "+-+0-+0+", "+-+0-++-", "+-+0-++0", "+-+0-+++", "+-+00---", "+-+00--0",
        "+-+00--+", "+-+00-0-", "+-+00-00", "+-+00-0+", "+-+00-+-", "+-+00-+0", "+-+00-++",
        "+-+000--", "+-+000-0", "+-+000-+", "+-+0000-", "+-+00000", "+-+0000+", "+-+000+-",
        "+-+000+0", "+-+000++", "+-+00+--", "+-+00+-0", "+-+00+-+", "+-+00+0-", "+-+00+00",
        "+-+00+0+", "+-+00++-", "+-+00++0", "+-+00+++", "+-+0+---", "+-+0+--0", "+-+0+--+",
        "+-+0+-0-", "+-+0+-00", "+-+0+-0+", "+-+0+-+-", "+-+0+-+0", "+-+0+-++", "+-+0+0--",
        "+-+0+0-0", "+-+0+0-+", "+-+0+00-", "+-+0+000", "+-+0+00+", "+-+0+0+-", "+-+0+0+0",
        "+-+0+0++", "+-+0++--", "+-+0++-0", "+-+0++-+", "+-+0++0-", "+-+0++00", "+-+0++0+",
        "+-+0+++-", "+-+0+++0", "+-+0++++", "+-++----", "+-++---0", "+-++---+", "+-++--0-",
        "+-++--00", "+-++--0+", "+-++--+-", "+-++--+0", "+-++--++", "+-++-0--", "+-++-0-0",
        "+-++-0-+", "+-++-00-", "+-++-000", "+-++-00+", "+-++-0+-", "+-++-0+0", "+-++-0++",
        "+-++-+--", "+-++-+-0", "+-++-+-+", "+-++-+0-", "+-++-+00", "+-++-+0+", "+-++-++-",
        "+-++-++0", "+-++-+++", "+-++0---", "+-++0--0", "+-++0--+", "+-++0-0-", "+-++0-00",
        "+-++0-0+", "+-++0-+-", "+-++0-+0", "+-++0-++", "+-++00--", "+-++00-0", "+-++00-+",
        "+-++000-", "+-++0000", "+-++000+", "+-++00+-", "+-++00+0", "+-++00++", "+-++0+--",
        "+-++0+-0", "+-++0+-+", "+-++0+0-", "+-++0+00", "+-++0+0+", "+-++0++-", "+-++0++0",
        "+-++0+++", "+-+++---", "+-+++--0", "+-+++--+", "+-+++-0-", "+-+++-00", "+-+++-0+",
        "+-+++-+-", "+-+++-+0", "+-+++-++", "+-+++0--", "+-+++0-0", "+-+++0-+", "+-+++00-",
        "+-+++000", "+-+++00+", "+-+++0+-", "+-+++0+0", "+-+++0++", "+-++++--", "+-++++-0",
        "+-++++-+", "+-++++0-", "+-++++00", "+-++++0+", "+-+++++-", "+-+++++0", "+-++++++",
        "+0------", "+0-----0", "+0-----+", "+0----0-", "+0----00", "+0----0+", "+0----+-",
        "+0----+0", "+0----++", "+0---0--", "+0---0-0", "+0---0-+", "+0---00-", "+0---000",
        "+0---00+", "+0---0+-", "+0---0+0", "+0---0++", "+0---+--", "+0---+-0", "+0---+-+",
        "+0---+0-", "+0---+00", "+0---+0+", "+0---++-", "+0---++0", "+0---+++", "+0--0---",
        "+0--0--0", "+0--0--+", "+0--0-0-", "+0--0-00", "+0--0-0+", "+0--0-+-", "+0--0-+0",
        "+0--0-++", "+0--00--", "+0--00-0", "+0--00-+", "+0--000-", "+0--0000", "+0--000+",
        "+0--00+-", "+0--00+0", "+0--00++", "+0--0+--", "+0--0+-0", "+0--0+-+", "+0--0+0-",
        "+0--0+00", "+0--0+0+", "+0--0++-", "+0--0++0", "+0--0+++", "+0--+---", "+0--+--0",
        "+0--+--+", "+0--+-0-", "+0--+-00", "+0--+-0+", "+0--+-+-", "+0--+-+0", "+0--+-++",
        "+0--+0--", "+0--+0-0", "+0--+0-+", "+0--+00-", "+0--+000", "+0--+00+", "+0--+0+-",
        "+0--+0+0", "+0--+0++", "+0--++--", "+0--++-0", "+0--++-+", "+0--++0-", "+0--++00",
        "+0--++0+", "+0--+++-", "+0--+++0", "+0--++++", "+0-0----", "+0-0---0", "+0-0---+",
        "+0-0--0-", "+0-0--00", "+0-0--0+", "+0-0--+-", "+0-0--+0", "+0-0--++", "+0-0-0--",
        "+0-0-0-0", "+0-0-0-+", "+0-0-00-", "+0-0-000", "+0-0-00+", "+0-0-0+-", "+0-0-0+0",
        "+0-0-0++", "+0-0-+--", "+0-0-+-0", "+0-0-+-+", "+0-0-+0-", "+0-0-+00", "+0-0-+0+",
        "+0-0-++-", "+0-0-++0", "+0-0-+++", "+0-00---", "+0-00--0", "+0-00--+", "+0-00-0-",
        "+0-00-00", "+0-00-0+", "+0-00-+-", "+0-00-+0", "+0-00-++", "+0-000--", "+0-000-0",
        "+0-000-+", "+0-0000-", "+0-00000", "+0-0000+", "+0-000+-", "+0-000+0", "+0-000++",
        "+0-00+--", "+0-00+-0", "+0-00+-+", "+0-00+0-", "+0-00+00", "+0-00+0+", "+0-00++-",
        "+0-00++0", "+0-00+++", "+0-0+---", "+0-0+--0", "+0-0+--+", "+0-0+-0-", "+0-0+-00",
        "+0-0+-0+", "+0-0+-+-", "+0-0+-+0", "+0-0+-++", "+0-0+0--", "+0-0+0-0", "+0-0+0-+",
        "+0-0+00-", "+0-0+000", "+0-0+00+", "+0-0+0+-", "+0-0+0+0", "+0-0+0++", "+0-0++--",
        "+0-0++-0", "+0-0++-+", "+0-0++0-", "+0-0++00", "+0-0++0+", "+0-0+++-", "+0-0+++0",
        "+0-0++++", "+0-+----", "+0-+---0", "+0-+---+", "+0-+--0-", "+0-+--00", "+0-+--0+",
        "+0-+--+-", "+0-+--+0", "+0-+--++", "+0-+-0--", "+0-+-0-0", "+0-+-0-+", "+0-+-00-",
        "+0-+-000", "+0-+-00+", "+0-+-0+-", "+0-+-0+0", "+0-+-0++", "+0-+-+--", "+0-+-+-0",
        "+0-+-+-+", "+0-+-+0-", "+0-+-+00", "+0-+-+0+", "+0-+-++-", "+0-+-++0", "+0-+-+++",
        "+0-+0---", "+0-+0--0", "+0-+0--+", "+0-+0-0-", "+0-+0-00", "+0-+0-0+", "+0-+0-+-",
        "+0-+0-+0", "+0-+0-++", "+0-+00--", "+0-+00-0", "+0-+00-+", "+0-+000-", "+0-+0000",
        "+0-+000+", "+0-+00+-", "+0-+00+0", "+0-+00++", "+0-+0+--", "+0-+0+-0", "+0-+0+-+",
        "+0-+0+0-", "+0-+0+00", "+0-+0+0+", "+0-+0++-", "+0-+0++0", "+0-+0+++", "+0-++---",
        "+0-++--0", "+0-++--+", "+0-++-0-", "+0-++-00", "+0-++-0+", "+0-++-+-", "+0-++-+0",
        "+0-++-++", "+0-++0--", "+0-++0-0", "+0-++0-+", "+0-++00-", "+0-++000", "+0-++00+",
        "+0-++0+-", "+0-++0+0", "+0-++0++", "+0-+++--", "+0-+++-0", "+0-+++-+", "+0-+++0-",
        "+0-+++00", "+0-+++0+", "+0-++++-", "+0-++++0", "+0-+++++", "+00-----", "+00----0",
        "+00----+", "+00---0-", "+00---00", "+00---0+", "+00---+-", "+00---+0", "+00---++",
        "+00--0--", "+00--0-0", "+00--0-+", "+00--00-", "+00--000", "+00--00+", "+00--0+-",
        "+00--0+0", "+00--0++", "+00--+--", "+00--+-0", "+00--+-+", "+00--+0-", "+00--+00",
        "+00--+0+", "+00--++-", "+00--++0", "+00--+++", "+00-0---", "+00-0--0", "+00-0--+",
        "+00-0-0-", "+00-0-00", "+00-0-0+", "+00-0-+-", "+00-0-+0", "+00-0-++", "+00-00--",
        "+00-00-0", "+00-00-+", "+00-000-", "+00-0000", "+00-000+", "+00-00+-", "+00-00+0",
        "+00-00++", "+00-0+--", "+00-0+-0", "+00-0+-+", "+00-0+0-", "+00-0+00", "+00-0+0+",
        "+00-0++-", "+00-0++0", "+00-0+++", "+00-+---", "+00-+--0", "+00-+--+", "+00-+-0-",
        "+00-+-00", "+00-+-0+", "+00-+-+-", "+00-+-+0", "+00-+-++", "+00-+0--", "+00-+0-0",
        "+00-+0-+", "+00-+00-", "+00-+000", "+00-+00+", "+00-+0+-", "+00-+0+0", "+00-+0++",
        "+00-++--", "+00-++-0", "+00-++-+", "+00-++0-", "+00-++00", "+00-++0+", "+00-+++-",
        "+00-+++0", "+00-++++", "+000----", "+000---0", "+000---+", "+000--0-", "+000--00",
        "+000--0+", "+000--+-", "+000--+0", "+000--++", "+000-0--", "+000-0-0", "+000-0-+",
        "+000-00-", "+000-000", "+000-00+", "+000-0+-", "+000-0+0", "+000-0++", "+000-+--",
        "+000-+-0", "+000-+-+", "+000-+0-", "+000-+00", "+000-+0+", "+000-++-", "+000-++0",
        "+000-+++", "+0000---", "+0000--0", "+0000--+", "+0000-0-", "+0000-00", "+0000-0+",
        "+0000-+-", "+0000-+0", "+0000-++", "+00000--", "+00000-0", "+00000-+", "+000000-",
        "+0000000", "+000000+", "+00000+-", "+00000+0", "+00000++", "+0000+--", "+0000+-0",
        "+0000+-+", "+0000+0-", "+0000+00", "+0000+0+", "+0000++-", "+0000++0", "+0000+++",
        "+000+---", "+000+--0", "+000+--+", "+000+-0-", "+000+-00", "+000+-0+", "+000+-+-",
        "+000+-+0", "+000+-++", "+000+0--", "+000+0-0", "+000+0-+", "+000+00-", "+000+000",
        "+000+00+", "+000+0+-", "+000+0+0", "+000+0++", "+000++--", "+000++-0", "+000++-+",
        "+000++0-", "+000++00", "+000++0+", "+000+++-", "+000+++0", "+000++++", "+00+----",
        "+00+---0", "+00+---+", "+00+--0-", "+00+--00", "+00+--0+", "+00+--+-", "+00+--+0",
        "+00+--++", "+00+-0--", "+00+-0-0", "+00+-0-+", "+00+-00-", "+00+-000", "+00+-00+",
        "+00+-0+-", "+00+-0+0", "+00+-0++", "+00+-+--", "+00+-+-0", "+00+-+-+", "+00+-+0-",
        "+00+-+00", "+00+-+0+", "+00+-++-", "+00+-++0", "+00+-+++", "+00+0---", "+00+0--0",
        "+00+0--+", "+00+0-0-", "+00+0-00", "+00+0-0+", "+00+0-+-", "+00+0-+0", "+00+0-++",
        "+00+00--", "+00+00-0", "+00+00-+", "+00+000-", "+00+0000", "+00+000+", "+00+00+-",
        "+00+00+0", "+00+00++", "+00+0+--", "+00+0+-0", "+00+0+-+", "+00+0+0-", "+00+0+00",
        "+00+0+0+", "+00+0++-", "+00+0++0", "+00+0+++", "+00++---", "+00++--0", "+00++--+",
        "+00++-0-", "+00++-00", "+00++-0+", "+00++-+-", "+00++-+0", "+00++-++", "+00++0--",
        "+00++0-0", "+00++0-+", "+00++00-", "+00++000", "+00++00+", "+00++0+-", "+00++0+0",
        "+00++0++", "+00+++--", "+00+++-0", "+00+++-+", "+00+++0-", "+00+++00", "+00+++0+",
        "+00++++-", "+00++++0", "+00+++++", "+0+-----", "+0+----0", "+0+----+", "+0+---0-",
        "+0+---00", "+0+---0+", "+0+---+-", "+0+---+0", "+0+---++", "+0+--0--", "+0+--0-0",
        "+0+--0-+", "+0+--00-", "+0+--000", "+0+--00+", "+0+--0+-", "+0+--0+0", "+0+--0++",
        "+0+--+--", "+0+--+-0", "+0+--+-+", "+0+--+0-", "+0+--+00", "+0+--+0+", "+0+--++-",
        "+0+--++0", "+0+--+++", "+0+-0---", "+0+-0--0", "+0+-0--+", "+0+-0-0-", "+0+-0-00",
        "+0+-0-0+", "+0+-0-+-", "+0+-0-+0", "+0+-0-++", "+0+-00--", "+0+-00-0", "+0+-00-+",
        "+0+-000-", "+0+-0000", "+0+-000+", "+0+-00+-", "+0+-00+0", "+0+-00++", "+0+-0+--",
        "+0+-0+-0", "+0+-0+-+", "+0+-0+0-", "+0+-0+00", "+0+-0+0+", "+0+-0++-", "+0+-0++0",
        "+0+-0+++", "+0+-+---", "+0+-+--0", "+0+-+--+", "+0+-+-0-", "+0+-+-00", "+0+-+-0+",
        "+0+-+-+-", "+0+-+-+0", "+0+-+-++", "+0+-+0--", "+0+-+0-0", "+0+-+0-+", "+0+-+00-",
        "+0+-+000", "+0+-+00+", "+0+-+0+-", "+0+-+0+0", "+0+-+0++", "+0+-++--", "+0+-++-0",
        "+0+-++-+", "+0+-++0-", "+0+-++00", "+0+-++0+", "+0+-+++-", "+0+-+++0", "+0+-++++",
        "+0+0----", "+0+0---0", "+0+0---+", "+0+0--0-", "+0+0--00", "+0+0--0+", "+0+0--+-",
        "+0+0--+0", "+0+0--++", "+0+0-0--", "+0+0-0-0", "+0+0-0-+", "+0+0-00-", "+0+0-000",
        "+0+0-00+", "+0+0-0+-", "+0+0-0+0", "+0+0-0++", "+0+0-+--", "+0+0-+-0", "+0+0-+-+",
        "+0+0-+0-", "+0+0-+00", "+0+0-+0+", "+0+0-++-", "+0+0-++0", "+0+0-+++", "+0+00---",
        "+0+00--0", "+0+00--+", "+0+00-0-", "+0+00-00", "+0+00-0+", "+0+00-+-", "+0+00-+0",
        "+0+00-++", "+0+000--", "+0+000-0", "+0+000-+", "+0+0000-", "+0+00000", "+0+0000+",
        "+0+000+-", "+0+000+0", "+0+000++", "+0+00+--", "+0+00+-0", "+0+00+-+", "+0+00+0-",
        "+0+00+00", "+0+00+0+", "+0+00++-", "+0+00++0", "+0+00+++", "+0+0+---", "+0+0+--0",
        "+0+0+--+", "+0+0+-0-", "+0+0+-00", "+0+0+-0+", "+0+0+-+-", "+0+0+-+0", "+0+0+-++",
        "+0+0+0--", "+0+0+0-0", "+0+0+0-+", "+0+0+00-", "+0+0+000", "+0+0+00+", "+0+0+0+-",
        "+0+0+0+0", "+0+0+0++", "+0+0++--", "+0+0++-0", "+0+0++-+", "+0+0++0-", "+0+0++00",
        "+0+0++0+", "+0+0+++-", "+0+0+++0", "+0+0++++", "+0++----", "+0++---0", "+0++---+",
        "+0++--0-", "+0++--00", "+0++--0+", "+0++--+-", "+0++--+0", "+0++--++", "+0++-0--",
        "+0++-0-0", "+0++-0-+", "+0++-00-", "+0++-000", "+0++-00+", "+0++-0+-", "+0++-0+0",
        "+0++-0++", "+0++-+--", "+0++-+-0", "+0++-+-+", "+0++-+0-", "+0++-+00", "+0++-+0+",
        "+0++-++-", "+0++-++0", "+0++-+++", "+0++0---", "+0++0--0", "+0++0--+", "+0++0-0-",
        "+0++0-00", "+0++0-0+", "+0++0-+-", "+0++0-+0", "+0++0-++", "+0++00--", "+0++00-0",
        "+0++00-+", "+0++000-", "+0++0000", "+0++000+", "+0++00+-", "+0++00+0", "+0++00++",
        "+0++0+--", "+0++0+-0", "+0++0+-+", "+0++0+0-", "+0++0+00", "+0++0+0+", "+0++0++-",
        "+0++0++0", "+0++0+++", "+0+++---", "+0+++--0", "+0+++--+", "+0+++-0-", "+0+++-00",
        "+0+++-0+", "+0+++-+-", "+0+++-+0", "+0+++-++", "+0+++0--", "+0+++0-0", "+0+++0-+",
        "+0+++00-", "+0+++000", "+0+++00+", "+0+++0+-", "+0+++0+0", "+0+++0++", "+0++++--",
        "+0++++-0", "+0++++-+", "+0++++0-", "+0++++00", "+0++++0+", "+0+++++-", "+0+++++0",
        "+0++++++", "++------", "++-----0", "++-----+", "++----0-", "++----00", "++----0+",
        "++----+-", "++----+0", "++----++", "++---0--", "++---0-0", "++---0-+", "++---00-",
        "++---000", "++---00+", "++---0+-", "++---0+0", "++---0++", "++---+--", "++---+-0",
        "++---+-+", "++---+0-", "++---+00", "++---+0+", "++---++-", "++---++0", "++---+++",
        "++--0---", "++--0--0", "++--0--+", "++--0-0-", "++--0-00", "++--0-0+", "++--0-+-",
        "++--0-+0", "++--0-++", "++--00--", "++--00-0", "++--00-+", "++--000-", "++--0000",
        "++--000+", "++--00+-", "++--00+0", "++--00++", "++--0+--", "++--0+-0", "++--0+-+",
        "++--0+0-", "++--0+00", "++--0+0+", "++--0++-", "++--0++0", "++--0+++", "++--+---",
        "++--+--0", "++--+--+", "++--+-0-", "++--+-00", "++--+-0+", "++--+-+-", "++--+-+0",
        "++--+-++", "++--+0--", "++--+0-0", "++--+0-+", "++--+00-", "++--+000", "++--+00+",
        "++--+0+-", "++--+0+0", "++--+0++", "++--++--", "++--++-0", "++--++-+", "++--++0-",
        "++--++00", "++--++0+", "++--+++-", "++--+++0", "++--++++", "++-0----", "++-0---0",
        "++-0---+", "++-0--0-", "++-0--00", "++-0--0+", "++-0--+-", "++-0--+0", "++-0--++",
        "++-0-0--", "++-0-0-0", "++-0-0-+", "++-0-00-", "++-0-000", "++-0-00+", "++-0-0+-",
        "++-0-0+0", "++-0-0++", "++-0-+--", "++-0-+-0", "++-0-+-+", "++-0-+0-", "++-0-+00",
        "++-0-+0+", "++-0-++-", "++-0-++0", "++-0-+++", "++-00---", "++-00--0", "++-00--+",
        "++-00-0-", "++-00-00", "++-00-0+", "++-00-+-", "++-00-+0", "++-00-++", "++-000--",
        "++-000-0", "++-000-+", "++-0000-", "++-00000", "++-0000+", "++-000+-", "++-000+0",
        "++-000++", "++-00+--", "++-00+-0", "++-00+-+", "++-00+0-", "++-00+00", "++-00+0+",
        "++-00++-", "++-00++0", "++-00+++", "++-0+---", "++-0+--0", "++-0+--+", "++-0+-0-",
        "++-0+-00", "++-0+-0+", "++-0+-+-", "++-0+-+0", "++-0+-++", "++-0+0--", "++-0+0-0",
        "++-0+0-+", "++-0+00-", "++-0+000", "++-0+00+", "++-0+0+-", "++-0+0+0", "++-0+0++",
        "++-0++--", "++-0++-0", "++-0++-+", "++-0++0-", "++-0++00", "++-0++0+", "++-0+++-",
        "++-0+++0", "++-0++++", "++-+----", "++-+---0", "++-+---+", "++-+--0-", "++-+--00",
        "++-+--0+", "++-+--+-", "++-+--+0", "++-+--++", "++-+-0--", "++-+-0-0", "++-+-0-+",
        "++-+-00-", "++-+-000", "++-+-00+", "++-+-0+-", "++-+-0+0", "++-+-0++", "++-+-+--",
        "++-+-+-0", "++-+-+-+", "++-+-+0-", "++-+-+00", "++-+-+0+", "++-+-++-", "++-+-++0",
        "++-+-+++", "++-+0---", "++-+0--0", "++-+0--+", "++-+0-0-", "++-+0-00", "++-+0-0+",
        "++-+0-+-", "++-+0-+0", "++-+0-++", "++-+00--", "++-+00-0", "++-+00-+", "++-+000-",
        "++-+0000", "++-+000+", "++-+00+-", "++-+00+0", "++-+00++", "++-+0+--", "++-+0+-0",
        "++-+0+-+", "++-+0+0-", "++-+0+00", "++-+0+0+", "++-+0++-", "++-+0++0", "++-+0+++",
        "++-++---", "++-++--0", "++-++--+", "++-++-0-", "++-++-00", "++-++-0+", "++-++-+-",
        "++-++-+0", "++-++-++", "++-++0--", "++-++0-0", "++-++0-+", "++-++00-", "++-++000",
        "++-++00+", "++-++0+-", "++-++0+0", "++-++0++", "++-+++--", "++-+++-0", "++-+++-+",
        "++-+++0-", "++-+++00", "++-+++0+", "++-++++-", "++-++++0", "++-+++++", "++0-----",
        "++0----0", "++0----+", "++0---0-", "++0---00", "++0---0+", "++0---+-", "++0---+0",
        "++0---++", "++0--0--", "++0--0-0", "++0--0-+", "++0--00-", "++0--000", "++0--00+",
        "++0--0+-", "++0--0+0", "++0--0++", "++0--+--", "++0--+-0", "++0--+-+", "++0--+0-",
        "++0--+00", "++0--+0+", "++0--++-", "++0--++0", "++0--+++", "++0-0---", "++0-0--0",
        "++0-0--+", "++0-0-0-", "++0-0-00", "++0-0-0+", "++0-0-+-", "++0-0-+0", "++0-0-++",
        "++0-00--", "++0-00-0", "++0-00-+", "++0-000-", "++0-0000", "++0-000+", "++0-00+-",
        "++0-00+0", "++0-00++", "++0-0+--", "++0-0+-0", "++0-0+-+", "++0-0+0-", "++0-0+00",
        "++0-0+0+", "++0-0++-", "++0-0++0", "++0-0+++", "++0-+---", "++0-+--0", "++0-+--+",
        "++0-+-0-", "++0-+-00", "++0-+-0+", "++0-+-+-", "++0-+-+0", "++0-+-++", "++0-+0--",
        "++0-+0-0", "++0-+0-+", "++0-+00-", "++0-+000", "++0-+00+", "++0-+0+-", "++0-+0+0",
        "++0-+0++", "++0-++--", "++0-++-0", "++0-++-+", "++0-++0-", "++0-++00", "++0-++0+",
        "++0-+++-", "++0-+++0", "++0-++++", "++00----", "++00---0", "++00---+", "++00--0-",
        "++00--00", "++00--0+", "++00--+-", "++00--+0", "++00--++", "++00-0--", "++00-0-0",
        "++00-0-+", "++00-00-", "++00-000", "++00-00+", "++00-0+-", "++00-0+0", "++00-0++",
        "++00-+--", "++00-+-0", "++00-+-+", "++00-+0-", "++00-+00", "++00-+0+", "++00-++-",
        "++00-++0", "++00-+++", "++000---", "++000--0", "++000--+", "++000-0-", "++000-00",
        "++000-0+", "++000-+-", "++000-+0", "++000-++", "++0000--", "++0000-0", "++0000-+",
        "++00000-", "++000000", "++00000+", "++0000+-", "++0000+0", "++0000++", "++000+--",
        "++000+-0", "++000+-+", "++000+0-", "++000+00", "++000+0+", "++000++-", "++000++0",
        "++000+++", "++00+---", "++00+--0", "++00+--+", "++00+-0-", "++00+-00", "++00+-0+",
        "++00+-+-", "++00+-+0", "++00+-++", "++00+0--", "++00+0-0", "++00+0-+", "++00+00-",
        "++00+000", "++00+00+", "++00+0+-", "++00+0+0", "++00+0++", "++00++--", "++00++-0",
        "++00++-+", "++00++0-", "++00++00", "++00++0+", "++00+++-", "++00+++0", "++00++++",
        "++0+----", "++0+---0", "++0+---+", "++0+--0-", "++0+--00", "++0+--0+", "++0+--+-",
        "++0+--+0", "++0+--++", "++0+-0--", "++0+-0-0", "++0+-0-+", "++0+-00-", "++0+-000",
        "++0+-00+", "++0+-0+-", "++0+-0+0", "++0+-0++", "++0+-+--", "++0+-+-0", "++0+-+-+",
        "++0+-+0-", "++0+-+00", "++0+-+0+", "++0+-++-", "++0+-++0", "++0+-+++", "++0+0---",
        "++0+0--0", "++0+0--+", "++0+0-0-", "++0+0-00", "++0+0-0+", "++0+0-+-", "++0+0-+0",
        "++0+0-++", "++0+00--", "++0+00-0", "++0+00-+", "++0+000-", "++0+0000", "++0+000+",
        "++0+00+-", "++0+00+0",
    ];
    for test_subject in 0..=3000 {
        assert!(util::cbb::int_to_bal_ternary(test_subject) == RESULTS[test_subject as usize]);
    }
}
