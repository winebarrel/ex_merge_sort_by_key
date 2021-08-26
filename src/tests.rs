use super::reverse_sort_by_key;
use super::sort_by_key;
use indoc::indoc;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::str;

static CSV: &str = indoc! {"
    0,Golf,189
    1,Yankee,157
    10,Delta,170
    11,Zulu,118
    12,Sierra,186
    13,Charlie,195
    14,Alpha,149
    15,November,190
    16,Tango,194
    17,Lima,121
    18,Victor,163
    19,Romeo,191
    2,Uniform,158
    20,Foxtrot,188
    21,Bravo,111
    22,Kilo,161
    23,X-ray,167
    24,Oscar,141
    25,Quebec,179
    3,Juliet,178
    4,Papa,138
    5,Mike,110
    6,Whiskey,116
    7,Hotel,137
    8,Echo,132
    9,India,125\n
"};

#[test]
fn test_sort_in_buf1() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 1024, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[1].to_string()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            14,Alpha,149
            21,Bravo,111
            13,Charlie,195
            10,Delta,170
            8,Echo,132
            20,Foxtrot,188
            0,Golf,189
            7,Hotel,137
            9,India,125
            3,Juliet,178
            22,Kilo,161
            17,Lima,121
            5,Mike,110
            15,November,190
            24,Oscar,141
            4,Papa,138
            25,Quebec,179
            19,Romeo,191
            12,Sierra,186
            16,Tango,194
            2,Uniform,158
            18,Victor,163
            6,Whiskey,116
            23,X-ray,167
            1,Yankee,157
            11,Zulu,118
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_in_buf2() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 1024, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[2].to_string()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            5,Mike,110
            21,Bravo,111
            6,Whiskey,116
            11,Zulu,118
            17,Lima,121
            9,India,125
            8,Echo,132
            7,Hotel,137
            4,Papa,138
            24,Oscar,141
            14,Alpha,149
            1,Yankee,157
            2,Uniform,158
            22,Kilo,161
            18,Victor,163
            23,X-ray,167
            10,Delta,170
            3,Juliet,178
            25,Quebec,179
            12,Sierra,186
            20,Foxtrot,188
            0,Golf,189
            15,November,190
            19,Romeo,191
            16,Tango,194
            13,Charlie,195
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_reverse_sort_in_buf() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    reverse_sort_by_key(fin, fout, 1024, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[1].to_string()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            11,Zulu,118
            1,Yankee,157
            23,X-ray,167
            6,Whiskey,116
            18,Victor,163
            2,Uniform,158
            16,Tango,194
            12,Sierra,186
            19,Romeo,191
            25,Quebec,179
            4,Papa,138
            24,Oscar,141
            15,November,190
            5,Mike,110
            17,Lima,121
            22,Kilo,161
            3,Juliet,178
            9,India,125
            7,Hotel,137
            0,Golf,189
            20,Foxtrot,188
            8,Echo,132
            10,Delta,170
            13,Charlie,195
            21,Bravo,111
            14,Alpha,149
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_by_num_in_buf() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 1024, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[0].parse::<i32>().unwrap()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            0,Golf,189
            1,Yankee,157
            2,Uniform,158
            3,Juliet,178
            4,Papa,138
            5,Mike,110
            6,Whiskey,116
            7,Hotel,137
            8,Echo,132
            9,India,125
            10,Delta,170
            11,Zulu,118
            12,Sierra,186
            13,Charlie,195
            14,Alpha,149
            15,November,190
            16,Tango,194
            17,Lima,121
            18,Victor,163
            19,Romeo,191
            20,Foxtrot,188
            21,Bravo,111
            22,Kilo,161
            23,X-ray,167
            24,Oscar,141
            25,Quebec,179
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_reverse_sort_by_num_in_buf() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    reverse_sort_by_key(fin, fout, 1024, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[0].parse::<i32>().unwrap()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            25,Quebec,179
            24,Oscar,141
            23,X-ray,167
            22,Kilo,161
            21,Bravo,111
            20,Foxtrot,188
            19,Romeo,191
            18,Victor,163
            17,Lima,121
            16,Tango,194
            15,November,190
            14,Alpha,149
            13,Charlie,195
            12,Sierra,186
            11,Zulu,118
            10,Delta,170
            9,India,125
            8,Echo,132
            7,Hotel,137
            6,Whiskey,116
            5,Mike,110
            4,Papa,138
            3,Juliet,178
            2,Uniform,158
            1,Yankee,157
            0,Golf,189
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_using_file1() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 10, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[1].to_string()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            14,Alpha,149
            21,Bravo,111
            13,Charlie,195
            10,Delta,170
            8,Echo,132
            20,Foxtrot,188
            0,Golf,189
            7,Hotel,137
            9,India,125
            3,Juliet,178
            22,Kilo,161
            17,Lima,121
            5,Mike,110
            15,November,190
            24,Oscar,141
            4,Papa,138
            25,Quebec,179
            19,Romeo,191
            12,Sierra,186
            16,Tango,194
            2,Uniform,158
            18,Victor,163
            6,Whiskey,116
            23,X-ray,167
            1,Yankee,157
            11,Zulu,118
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_using_file2() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 10, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[2].to_string()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            5,Mike,110
            21,Bravo,111
            6,Whiskey,116
            11,Zulu,118
            17,Lima,121
            9,India,125
            8,Echo,132
            7,Hotel,137
            4,Papa,138
            24,Oscar,141
            14,Alpha,149
            1,Yankee,157
            2,Uniform,158
            22,Kilo,161
            18,Victor,163
            23,X-ray,167
            10,Delta,170
            3,Juliet,178
            25,Quebec,179
            12,Sierra,186
            20,Foxtrot,188
            0,Golf,189
            15,November,190
            19,Romeo,191
            16,Tango,194
            13,Charlie,195
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn reverse_test_sort_using_file() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    reverse_sort_by_key(fin, fout, 10, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[1].to_string()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            11,Zulu,118
            1,Yankee,157
            23,X-ray,167
            6,Whiskey,116
            18,Victor,163
            2,Uniform,158
            16,Tango,194
            12,Sierra,186
            19,Romeo,191
            25,Quebec,179
            4,Papa,138
            24,Oscar,141
            15,November,190
            5,Mike,110
            17,Lima,121
            22,Kilo,161
            3,Juliet,178
            9,India,125
            7,Hotel,137
            0,Golf,189
            20,Foxtrot,188
            8,Echo,132
            10,Delta,170
            13,Charlie,195
            21,Bravo,111
            14,Alpha,149
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_by_num_using_file() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 10, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[0].parse::<i32>().unwrap()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            0,Golf,189
            1,Yankee,157
            2,Uniform,158
            3,Juliet,178
            4,Papa,138
            5,Mike,110
            6,Whiskey,116
            7,Hotel,137
            8,Echo,132
            9,India,125
            10,Delta,170
            11,Zulu,118
            12,Sierra,186
            13,Charlie,195
            14,Alpha,149
            15,November,190
            16,Tango,194
            17,Lima,121
            18,Victor,163
            19,Romeo,191
            20,Foxtrot,188
            21,Bravo,111
            22,Kilo,161
            23,X-ray,167
            24,Oscar,141
            25,Quebec,179
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_reverse_sort_by_num_using_file() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "{}", CSV).unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    reverse_sort_by_key(fin, fout, 10, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[0].parse::<i32>().unwrap()
    })
    .unwrap();

    assert_eq!(
        indoc! {"
            25,Quebec,179
            24,Oscar,141
            23,X-ray,167
            22,Kilo,161
            21,Bravo,111
            20,Foxtrot,188
            19,Romeo,191
            18,Victor,163
            17,Lima,121
            16,Tango,194
            15,November,190
            14,Alpha,149
            13,Charlie,195
            12,Sierra,186
            11,Zulu,118
            10,Delta,170
            9,India,125
            8,Echo,132
            7,Hotel,137
            6,Whiskey,116
            5,Mike,110
            4,Papa,138
            3,Juliet,178
            2,Uniform,158
            1,Yankee,157
            0,Golf,189
        "},
        str::from_utf8(&buf).unwrap()
    );
}

#[test]
fn test_sort_empty() {
    let fin = tempfile::tempfile().unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 50, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[2].to_string()
    })
    .unwrap();

    assert_eq!("", str::from_utf8(&buf).unwrap());
}

#[test]
fn test_sort_one_line() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "0,Golf,189\n").unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 50, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[2].to_string()
    })
    .unwrap();

    assert_eq!("0,Golf,189\n", str::from_utf8(&buf).unwrap());
}

#[test]
fn test_sort_two_lines() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "0,Golf,189\n1,Yankee,157\n").unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 50, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[2].to_string()
    })
    .unwrap();

    assert_eq!("1,Yankee,157\n0,Golf,189\n", str::from_utf8(&buf).unwrap());
}

#[test]
fn test_sort_three_lines() {
    let mut fin = tempfile::tempfile().unwrap();
    write!(fin, "0,Golf,189\n1,Yankee,157\n10,Delta,170\n").unwrap();
    fin.seek(io::SeekFrom::Start(0)).unwrap();
    let mut buf = Vec::new();
    let fout = Box::new(&mut buf);

    sort_by_key(fin, fout, 50, |line| {
        let cols = line.split(',').collect::<Vec<&str>>();
        cols[2].to_string()
    })
    .unwrap();

    assert_eq!(
        "1,Yankee,157\n10,Delta,170\n0,Golf,189\n",
        str::from_utf8(&buf).unwrap()
    );
}
