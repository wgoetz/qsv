use newline_converter::dos2unix;
use serde_json::json;

use crate::workdir::Workdir;

#[test]
fn luau_map() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map").arg("inc").arg("number + 1").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_multiple_columns() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("newcol1,newcol2,newcol3")
        .arg("{number + 1, number + 2, number + 3}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "newcol1", "newcol2", "newcol3"],
        svec!["a", "13", "14", "15", "16"],
        svec!["b", "24", "25", "26", "27"],
        svec!["c", "72", "73", "74", "75"],
        svec!["d", "7", "8", "9", "10"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_idx() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("inc")
        .arg("number * _IDX")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "13"],
        svec!["b", "24", "48"],
        svec!["c", "72", "216"],
        svec!["d", "7", "28"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_aggregation() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("tot = (tot or 0) + Amount; return tot")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_aggregation_with_begin() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 0")
        .arg("tot = tot + Amount; return tot")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_aggregation_with_begin_end() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 0; gtotal = 0; amt_array = {}")
        .arg("amt_array[_IDX] = Amount; tot = tot + Amount; gtotal = gtotal + tot; return tot")
        .arg("--end")
        .arg(r#"return ("Min/Max: " .. math.min(unpack(amt_array)) .. "/" .. math.max(unpack(amt_array)) .. " Grand total of " .. _ROWCOUNT .. " rows: " .. gtotal)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end() {
    let wrk = Workdir::new("luau_embedded_begin_end");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg(
            "BEGIN {tot = 0; gtotal = 0; amt_array = {}}! amt_array[_IDX] = Amount; tot = tot + \
             Amount; gtotal = gtotal + tot; return tot\nEND {return (\"Min/Max: \" .. \
             math.min(unpack(amt_array)) .. \"/\" .. math.max(unpack(amt_array)) .. \" Grand \
             total of \" .. _ROWCOUNT .. \" rows: \" .. gtotal)}!",
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end_using_file() {
    let wrk = Workdir::new("luau_embedded_begin_end");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbeginend.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables, setup functions
    -- and load additional Lua libraries as required
    running_total = 0;
    grand_total = 0;
    amount_array = {};
    adjusted_array = {};

    function margin(x: number, y: number ): number
        return x * y;
    end

    function sum(numbers_array: table): number
        local sum: number = 0;
        for _, v in ipairs(numbers_array) do
            sum = sum + v;
        end
        return sum;
    end
}!


-- this is the MAIN script loop, which is executed for each row
-- note how we use the _IDX special variable to get the row index
amount_array[_IDX] = Amount;
running_total = running_total + Amount;

adjusted_array[_IDX] = Amount + margin(Amount, 0.25);

-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;


END {
    -- and this is the END block, which is executed once at the end
    grand_total = running_total;
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    adjusted_total = sum(adjusted_array);

    -- note how we use the _ROWCOUNT special variable to get the number of rows
    -- the value returned from the END script is sent to stderr
    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total} adjusted: {adjusted_total}`);
}!        
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testbeginend.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 116 adjusted: 145\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_register_lookup_table() {
    let wrk = Workdir::new("luau_register_lookup_table");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create(
        "us-states-lookup.csv",
        vec![
            svec![
                "Abbreviation",
                "Name",
                "Capital",
                "Population (2019)",
                "area (square miles)",
                "Sales Tax (2023)"
            ],
            svec!["NJ", "New Jersey", "Trenton", "8,882,190", "8,723", "7"],
            svec![
                "NM",
                "New Mexico",
                "Santa Fe",
                "2,096,829",
                "121,590",
                "5.13"
            ],
            svec!["NY", "New York", "Albany", "19,453,561", "54,555", "4"],
        ],
    );

    wrk.create_from_string(
        "testlookup.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
    csv_indexed = qsv_autoindex();

    us_states_lookup_headers = qsv_register_lookup("us_states", "us-states-lookup.csv", 1000)
    if not us_states_lookup_headers then
        qsv_break("Failed to register us_states lookup table")
    end

    -- note how we use the qsv_log function to log to the qsv log file
    qsv_log("debug", " _INDEX:", _INDEX, " _ROWCOUNT:", _ROWCOUNT, " csv_indexed:", csv_indexed)
    qsv_log("debug", "us_states_lookup_headers:", us_states_lookup_headers)
    qsv_log("debug", "us_states lookup table:", us_states)
    qsv_log("debug", "NY Capital:", us_states["NY"]["Capital"], " can be also: ", us_states.NY.Capital)
    -- start from the end of the CSV file, set _INDEX to _LASTROW
    _INDEX = _LASTROW;
}!


----------------------------------------------------------------------------
-- this is the MAIN script, which is executed for the row specified by _INDEX
-- As we are doing random access, to exit this loop, we need to set 
-- _INDEX to less than zero or greater than _LASTROW

amount_with_nytax = Amount + Amount * (us_states.NY["Sales Tax (2023)"] / 100);
amount_array[_INDEX] = amount_with_nytax;
running_total = running_total + amount_with_nytax;

qsv_log("warn", "logging from Luau script! running_total:", running_total, " _INDEX:", _INDEX)

-- we modify _INDEX to do random access on the CSV file, in this case going backwards
-- the MAIN script ends when _INDEX is less than zero or greater than _LASTROW
_INDEX = _INDEX - 1;

-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;


----------------------------------------------------------------------------
END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    grand_total = running_total;
    return `Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`;
}!
"#);

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testlookup.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["d", "7", "7.28"],
        svec!["c", "72", "82.16"],
        svec!["b", "24", "107.12"],
        svec!["a", "13", "120.64"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7.28/74.88 Grand total of 4 rows: 120.64\n";
    assert!(end.ends_with(expected_end));

    wrk.assert_success(&mut cmd);
}

// TODO: Ignore this test on Windows for now.
// Multiple attempts to fix this test on Windows
// have not been successful. The error is related to a file open
// issue, which is not easily reproducible.
#[cfg(not(target_os = "windows"))]
#[test]
fn luau_register_lookup_table_on_url() {
    let wrk = Workdir::new("luau_register_lookup_table_on_url");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testlookup.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
    csv_indexed = qsv_autoindex();

    us_states_lookup_headers = qsv_register_lookup("us_states", 
      "https://raw.githubusercontent.com/dathere/qsv/master/resources/test/us-states-lookup.csv", 100)
    if not us_states_lookup_headers then
        qsv_break("Failed to register us_states lookup table")
    end

    -- note how we use the qsv_log function to log to the qsv log file
    qsv_log("debug", " _INDEX:", _INDEX, " _ROWCOUNT:", _ROWCOUNT, " csv_indexed:", csv_indexed)
    qsv_log("debug", "us_states_lookup_headers:", us_states_lookup_headers)
    qsv_log("debug", "us_states lookup table:", us_states)
    qsv_log("debug", "NY Capital:", us_states["NY"]["Capital"], " can be also: ", us_states.NY.Capital)
    -- start from the end of the CSV file, set _INDEX to _LASTROW
    _INDEX = _LASTROW;
}!


----------------------------------------------------------------------------
-- this is the MAIN script, which is executed for the row specified by _INDEX
-- As we are doing random access, to exit this loop, we need to set 
-- _INDEX to less than zero or greater than _LASTROW

amount_with_nytax = Amount + Amount * (us_states.NY["Sales Tax (2023)"] / 100);
amount_array[_INDEX] = amount_with_nytax;
running_total = running_total + amount_with_nytax;

qsv_log("warn", "logging from Luau script! running_total:", running_total, " _INDEX:", _INDEX)

-- we modify _INDEX to do random access on the CSV file, in this case going backwards
-- the MAIN script ends when _INDEX is less than zero or greater than _LASTROW
_INDEX = _INDEX - 1;

-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;


----------------------------------------------------------------------------
END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    grand_total = running_total;
    return `Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`;
}!
"#);

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testlookup.luau")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    // Add a delay or ensure the file is closed properly
    std::thread::sleep(std::time::Duration::from_millis(500));

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["d", "7", "7.28"],
        svec!["c", "72", "82.16"],
        svec!["b", "24", "107.12"],
        svec!["a", "13", "120.64"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7.28/74.88 Grand total of 4 rows: 120.64\n";
    assert!(end.ends_with(expected_end));

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_register_lookup_table_on_dathere_url() {
    let wrk = Workdir::new("luau_register_lookup_table_on_dathere_url");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testlookup.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
    csv_indexed = qsv_autoindex();

    us_states_lookup_headers = qsv_register_lookup("us_states", 
      "dathere://us-states-example.csv", 100)
    if not us_states_lookup_headers then
        qsv_break("Failed to register us_states lookup table")
    end

    -- note how we use the qsv_log function to log to the qsv log file
    qsv_log("debug", " _INDEX:", _INDEX, " _ROWCOUNT:", _ROWCOUNT, " csv_indexed:", csv_indexed)
    qsv_log("debug", "us_states_lookup_headers:", us_states_lookup_headers)
    qsv_log("debug", "us_states lookup table:", us_states)
    qsv_log("debug", "NY Capital:", us_states["NY"]["Capital"], " can be also: ", us_states.NY.Capital)
    -- start from the end of the CSV file, set _INDEX to _LASTROW
    _INDEX = _LASTROW;
}!


----------------------------------------------------------------------------
-- this is the MAIN script, which is executed for the row specified by _INDEX
-- As we are doing random access, to exit this loop, we need to set 
-- _INDEX to less than zero or greater than _LASTROW

amount_with_nytax = Amount + Amount * (us_states.NY["Sales Tax (2023)"] / 100);
amount_array[_INDEX] = amount_with_nytax;
running_total = running_total + amount_with_nytax;

qsv_log("warn", "logging from Luau script! running_total:", running_total, " _INDEX:", _INDEX)

-- we modify _INDEX to do random access on the CSV file, in this case going backwards
-- the MAIN script ends when _INDEX is less than zero or greater than _LASTROW
_INDEX = _INDEX - 1;

-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;


----------------------------------------------------------------------------
END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    grand_total = running_total;
    return `Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`;
}!
"#);

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testlookup.luau")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["d", "7", "7.28"],
        svec!["c", "72", "82.16"],
        svec!["b", "24", "107.12"],
        svec!["a", "13", "120.64"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7.28/74.88 Grand total of 4 rows: 120.64\n";
    assert!(end.ends_with(expected_end));

    wrk.assert_success(&mut cmd);
}

#[test]
#[ignore = "Temporarily skip this as Boston CityScore Summary is currently unavailable"]
fn luau_register_lookup_table_ckan() {
    let wrk = Workdir::new("luau_register_lookup_ckan");
    wrk.create(
        "data.csv",
        vec![
            svec!["metric_name", "target_score"],
            svec!["POTHOLE ON-TIME %", "0.6"],
            svec!["ON-TIME PERMIT REVIEWS", "0.6"],
            svec!["BFD RESPONSE TIME", "0.6"],
            svec!["BPS ATTENDANCE", "0.6"],
        ],
    );

    wrk.create_from_string(
        "testlookup.luau",
        r#"
BEGIN {

    cityscore_headers = qsv_register_lookup("cityscore", "ckan://CityScore Summary?", 1000)
    if not cityscore_headers then
        qsv_break("Failed to register cityscore_headers lookup table")
    end

    function spairs(t, order)
        -- collect the keys
        local keys = {}
        for k in pairs(t) do keys[#keys+1] = k end
    
        -- if order function given, sort by it by passing the table and keys a, b,
        -- otherwise just sort the keys 
        if order then
            table.sort(keys, function(a,b) return order(t, a, b) end)
        else
            table.sort(keys)
        end
    
        -- return the iterator function
        local i = 0
        return function()
            i = i + 1
            if keys[i] then
                return keys[i], t[keys[i]]
            end
        end
    end

}!

-- this is the MAIN script

prev_month_score = cityscore[`{metric_name}`].previous_month_score

return prev_month_score;

END {
    sorted_headers = ""
    for k, v in spairs(cityscore_headers, function(t,a,b) return t[b] > t[a] end) do
        sorted_headers = sorted_headers .. v .. ","
    end
    return `{sorted_headers}`
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("previous_month_score")
        .arg("testlookup.luau")
        .arg("--ckan-api")
        .arg("https://data.boston.gov/api/3/action")
        .arg("data.csv");

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "previous_day_score,previous_month_score,previous_quarter_score,\
                        previous_week_score,score_calculated_ts,score_day_name,\
                        score_final_table_ts,\n";
    assert!(end.ends_with(expected_end));

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_writefile() {
    let wrk = Workdir::new("luau_writefile");
    wrk.create(
        "data.csv",
        vec![
            svec!["metric_name", "target_score"],
            svec!["POTHOLE ON-TIME %", "0.6"],
            svec!["ON-TIME PERMIT REVIEWS", "0.6"],
            svec!["BFD RESPONSE TIME", "0.6"],
            svec!["BPS ATTENDANCE", "0.6"],
        ],
    );

    wrk.create_from_string(
        "testwrite.luau",
        r#"
qsv_writefile("c:\windows\testfile.txt", `{metric_name} new target: {target_score * 2}\n`)
return target_score * 2;
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("new_target")
        .arg("file:testwrite.luau")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_qsv_break() {
    let wrk = Workdir::new("luau_qsv_break");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbreak.lua",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index
if (tonumber(Amount) > 25) then
    qsv_break("This is the break msg.");
else
    amount_array[_IDX] = Amount;
    running_total = running_total + Amount;
    grand_total = grand_total + running_total;

    -- running_total is the value we "map" to the "Running Total" column of each row
    return running_total;
end

END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_IDX - 1} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("testbreak.lua")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end =
        "This is the break msg.\nMin/Max: 13/24 Grand total of 2 rows: 50\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_qsv_skip() {
    let wrk = Workdir::new("luau_qsv_skip");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbreak.LUAU",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index
if (tonumber(Amount) > 25) then
    amount_array[_IDX] = 0;
    qsv_skip();
else
    amount_array[_IDX] = Amount;
    running_total = running_total + Amount;
    grand_total = grand_total + running_total;
end
-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_IDX - 1} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("testbreak.LUAU")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["d", "7", "44"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 0/24 Grand total of 3 rows: 94\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_qsv_loadcsv() {
    let wrk = Workdir::new("luau_qsv_loadcsv");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create(
        "datatoload.csv",
        vec![
            svec!["letter", "description"],
            svec!["a", "alpha"],
            svec!["b", "bravo"],
            svec!["c", "charlie"],
            svec!["d", "delta"],
        ],
    );

    wrk.create_from_string(
        "test_loadcsv.LUAU",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    qsv_loadcsv("datatoload_tbl", "datatoload.csv", "letter");
}!

-- this is the MAIN script, which is executed for each row
return datatoload_tbl[letter]["description"];

END {
    -- and this is the END block, which is executed once at the end
    return (`{_ROWCOUNT} rows`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("description")
        .arg("test_loadcsv.LUAU")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "description"],
        svec!["a", "13", "alpha"],
        svec!["b", "24", "bravo"],
        svec!["c", "72", "charlie"],
        svec!["d", "7", "delta"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "4 rows\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_qsv_cmd() {
    let wrk = Workdir::new("luau_qsv_cmd_test");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testqsvcmd.Lua",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
    qsv_writefile("count.txt", "_NEWFILE!");
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index

amount_array[_IDX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;
-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));

    qsv_cmd_output = qsv_cmd("count data.csv");
    qsv_writefile("count.txt", qsv_cmd_output.stdout);
    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_IDX - 1} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("testqsvcmd.Lua")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 3 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    let table_txt = wrk.read_to_string("count.txt").unwrap();
    let expected_table_txt = "4\n";
    assert_eq!(table_txt, expected_table_txt);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_qsv_invalid_shellcmd() {
    let wrk = Workdir::new("luau_qsv_invalid_shellcmd");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testqsvcmd.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
    qsv_writefile("echo.txt", "_NEWFILE!");
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index

amount_array[_IDX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;
-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));

    qsv_shellcmd_output = qsv_shellcmd("rm","-rf *");
    qsv_writefile("echo.txt", qsv_shellcmd_output.stderr);
    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_IDX - 1} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testqsvcmd.luau")
        .arg("data.csv");

    let output_stderr = wrk.output_stderr(&mut cmd);
    assert!(
        output_stderr.starts_with("<ERROR>")
            && output_stderr.contains("runtime error: Invalid shell command: \"rm\".")
    );

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_qsv_shellcmd() {
    let wrk = Workdir::new("luau_qsv_shellcmd");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testqsvcmd.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
    qsv_writefile("echo.txt", "_NEWFILE!");
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index

amount_array[_IDX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;
-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));

    qsv_shellcmd_output = qsv_shellcmd("echo","the quick brown fox jumped over the lazy dog");
    qsv_writefile("echo.txt", qsv_shellcmd_output.stdout);
    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_IDX - 1} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testqsvcmd.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let echo_text = wrk.read_to_string("echo.txt").unwrap();
    let expected_echo_text = "the quick brown fox jumped over the lazy dog";
    assert_eq!(
        dos2unix(&echo_text).trim_end(),
        dos2unix(expected_echo_text).trim_end()
    );

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_insertrecord() {
    let wrk = Workdir::new("luau_insertrecord");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbeginend.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index
amount_array[_IDX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;

qsv_insertrecord(`{letter}{_IDX}`, `{Amount}{_IDX}`, `{grand_total}`, `excess column, should not be inserted`)
-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));

    -- we insert a record at the end of the table with qsv_insertrecord
    -- with the grand total for Running Total, not counting the qsv_insertedrecords
    qsv_insertrecord(`Grand Total`, ``, `{grand_total}`);

    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testbeginend.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["a", "13", "13"],
        svec!["a1", "131", "13"],
        svec!["b", "24", "37"],
        svec!["b2", "242", "50"],
        svec!["c", "72", "109"],
        svec!["c3", "723", "159"],
        svec!["d", "7", "116"],
        svec!["d4", "74", "275"],
        svec!["Grand Total", "", "275"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_insertrecord_random_access() {
    let wrk = Workdir::new("luau_insertrecord_random_access");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testrandominsertrecord.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};

    qsv_autoindex();

    _INDEX = _LASTROW;
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index
amount_array[_IDX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;

qsv_insertrecord(`{letter}{_IDX}`, `{Amount}{_IDX}`, `{grand_total}`, `excess column, should not be inserted`)

_INDEX = _INDEX - 1;

-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));

    -- we insert a record at the end of the table with qsv_insertrecord
    -- with the grand total for Running Total, not counting the qsv_insertedrecords
    qsv_insertrecord(`Grand Total`, ``, `{grand_total}`);

    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testrandominsertrecord.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // let got2: String = wrk.stdout(&mut cmd);
    // println!("got2: {got2:?}");

    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["d", "7", "7"],
        svec!["d3", "73", "7"],
        svec!["c", "72", "79"],
        svec!["c2", "722", "86"],
        svec!["b", "24", "103"],
        svec!["b1", "241", "189"],
        svec!["a", "13", "116"],
        svec!["a0", "130", "305"],
        svec!["Grand Total", "", "305"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 305\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_test_string_interpolation_feature() {
    let wrk = Workdir::new("luau_embedded_begin_end");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "code digit"],
            svec!["a", "2"],
            svec!["b", "7"],
            svec!["c", "1"],
            svec!["d", "5"],
        ],
    );

    wrk.create_from_string(
        "testbeginend.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    code_array = {};
}!

-- this is the MAIN script, which is executed for each row
-- note how we use the _IDX special variable to get the row index
code_array[_IDX] = col["code digit"];
running_total = running_total + col["code digit"];
grand_total = grand_total + running_total;
-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    return(`The lock combination is {table.concat(code_array)}. Again, {table.concat(code_array, ", ")}.`)
}!        
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testbeginend.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "code digit", "Running Total"],
        svec!["a", "2", "2"],
        svec!["b", "7", "9"],
        svec!["c", "1", "10"],
        svec!["d", "5", "15"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "The lock combination is 2715. Again, 2, 7, 1, 5.\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end_and_beginend_options() {
    // when a main script has BEGIN/END blocks, and --begin/--end options are also specified,
    // the --begin/--end options take precedence and the embedded BEGIN/END blocks are ignored.
    let wrk = Workdir::new("luau_embedded_begin_end_options");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 1; gtotal = 0; amt_array = {}")
        .arg(
            "BEGIN {tot = 0; gtotal = 0; amt_array = {}}! amt_array[_IDX] = Amount; tot = tot + \
             Amount; gtotal = gtotal + tot; return tot\nEND {return (\"Min/Max: \" .. \
             math.min(unpack(amt_array)) .. \"/\" .. math.max(unpack(amt_array)) .. \" Grand \
             total of \" .. _ROWCOUNT .. \" rows: \" .. gtotal)}!",
        )
        .arg("--end")
        .arg(
            "return (\"Minimum/Maximum: \" .. math.min(unpack(amt_array)) .. \"/\" .. \
             math.max(unpack(amt_array)) .. \" Grand total of \" .. _ROWCOUNT .. \" rows: \" .. \
             gtotal)",
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "14"],
        svec!["b", "24", "38"],
        svec!["c", "72", "110"],
        svec!["d", "7", "117"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Minimum/Maximum: 7/72 Grand total of 4 rows: 279\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end_using_file_random_access() {
    let wrk = Workdir::new("luau_embedded_begin_end");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbeginend.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};

    -- note how we use the qsv_log function to log to the qsv log file
    qsv_log("debug", " _INDEX:", _INDEX, " _ROWCOUNT:", _ROWCOUNT)

    -- start from the end of the CSV file, set _INDEX to _LASTROW
    _INDEX = _LASTROW;
}!


----------------------------------------------------------------------------
-- this is the MAIN script, which is executed for the row specified by _INDEX
-- As we are doing random access, to exit this loop, we need to set 
-- _INDEX to less than zero or greater than _LASTROW

amount_array[_INDEX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;

qsv_log("warn", "logging from Luau script! running_total:", running_total, " _INDEX:", _INDEX)

-- we modify _INDEX to do random access on the CSV file, in this case going backwards
-- the MAIN script ends when _INDEX is less than zero or greater than _LASTROW
_INDEX = _INDEX - 1;

-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;


----------------------------------------------------------------------------
END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    return `Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`;
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testbeginend.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["d", "7", "7"],
        svec!["c", "72", "79"],
        svec!["b", "24", "103"],
        svec!["a", "13", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 305\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end_using_file_random_access_with_qsv_autoindex() {
    let wrk = Workdir::new("luau_embedded_qsv_index");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbeginend.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};

    -- here we call the qsv_autoindex() helper function to create/update an index
    -- so we can do random access on the CSV with the _INDEX special variable.
    -- qsv_autoindex() should only be called from the BEGIN block
    csv_indexed = qsv_autoindex();

    -- note how we use the qsv_log function to log to the qsv log file
    qsv_log("debug", " _INDEX:", _INDEX, " _ROWCOUNT:", _ROWCOUNT, " csv_indexed:", csv_indexed)

    -- start from the end of the CSV file, set _INDEX to _LASTROW
    _INDEX = _LASTROW;
}!


----------------------------------------------------------------------------
-- this is the MAIN script loop, which is executed for the row specified by _INDEX
-- As we are doing random access, to exit this loop, we need to set 
-- _INDEX to less than zero or greater than _LASTROW

amount_array[_INDEX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;

qsv_log("warn", "logging from Luau script! running_total:", running_total, " _INDEX:", _INDEX);

-- we modify _INDEX to do random access on the CSV file, in this case going backwards
-- here, the MAIN script ends when _INDEX is less than zero
_INDEX = _INDEX - 1;

-- running_total is the value we "map" to the "Running Total" column of the CURRENT row _INDEX
-- Note that the CURRENT row is still the _INDEX value when we entered this loop iteration,
-- not _INDEX - 1 which will become the next CURRENT row AFTER this loop iteration
return running_total;


----------------------------------------------------------------------------
END {
    -- and this is the END block, which is executed once at the end
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));

    -- note how we computed the Range by using the new Luau String interpolation feature
    -- which is similar to Python f-strings. We also used the special _ROWCOUNT variable here
    return (`Min/Max/Range: {min_amount}/{max_amount}/{max_amount - min_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("file:testbeginend.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["d", "7", "7"],
        svec!["c", "72", "79"],
        svec!["b", "24", "103"],
        svec!["a", "13", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max/Range: 7/72/65 Grand total of 4 rows: 305\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end_using_file_random_access_multiple_columns() {
    let wrk = Workdir::new("luau_embedded_multiple_columns");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbeginend.luau",
        r#"
BEGIN {
    -- this is the BEGIN block, which is executed once at the beginning
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amount_array = {};

    -- note how we use the qsv_log function to log to the qsv log file
    qsv_log("debug", " _INDEX:", _INDEX, " _ROWCOUNT:", _ROWCOUNT);

    -- start from the end of the CSV file, set _INDEX to _LASTROW
    _INDEX = _LASTROW;
}!


----------------------------------------------------------------------------
-- this is the MAIN script, which is executed for the row specified by _INDEX
-- As we are doing random access, to exit this loop, we need to set 
-- _INDEX to less than zero or greater than _LASTROW

amount_array[_INDEX] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;

qsv_log("warn", "logging from Luau script! running_total:", running_total, " _INDEX:", _INDEX);

-- we modify _INDEX to do random access on the CSV file, in this case going backwards
-- the MAIN script ends when _INDEX is less than zero or greater than _LASTROW
_INDEX = _INDEX - 1;

-- running_total is the value we "map" to the "Running Total" column of each row
return {running_total, running_total + (running_total * 0.1)};


----------------------------------------------------------------------------
END {
    -- and this is the END block, which is executed once at the end
    -- note how we use the _ROWCOUNT special variable to get the number of rows
    min_amount = math.min(unpack(amount_array));
    max_amount = math.max(unpack(amount_array));
    return (`Min/Max: {min_amount}/{max_amount} Grand total of {_ROWCOUNT} rows: {grand_total}`);
}!
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total, Running Total with 10%")
        .arg("file:testbeginend.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "letter",
            "Amount",
            "Running Total",
            "Running Total with 10%"
        ],
        svec!["d", "7", "7", "7.7"],
        svec!["c", "72", "79", "86.9"],
        svec!["b", "24", "103", "113.3"],
        svec!["a", "13", "116", "127.6"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 305\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_begin_end_and_luau_syntax() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "-24"],
            svec!["c", "-72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 0; gtotal = 0; amt_array = {}")
        .arg("amt_array[_IDX] = Amount; tot += if tonumber(Amount) < 0 then Amount * -1 else Amount; gtotal += tot; return tot")
        .arg("--end")
        .arg(r#"return ("Min/Max: " .. math.min(unpack(amt_array)) .. "/" .. math.max(unpack(amt_array)) .. " Grand total of " .. _ROWCOUNT .. " rows: " .. gtotal)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "-24", "37"],
        svec!["c", "-72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: -72/13 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_map_remap_with_qsv_coalesce() {
    let wrk = Workdir::new("luau_remap_coalesce");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "name_right"],
            svec!["1", "Artur A. Mosiyan", ""],
            svec!["2", "Eduard A. Aghabekyan", ""],
            svec!["3", "Արամայիս Աղաբեկյան", "Aramais E. Aghabekyan"],
            svec!["4", "Eleonora V. Avanesyan", ""],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("id,name")
        .arg("--remap")
        .arg("{id,qsv_coalesce(name_right,name)}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "name"],
        svec!["1", "Artur A. Mosiyan"],
        svec!["2", "Eduard A. Aghabekyan"],
        svec!["3", "Aramais E. Aghabekyan"],
        svec!["4", "Eleonora V. Avanesyan"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_map_math() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("div")
        .arg("math.floor(number / 2)")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "div"],
        svec!["a", "13", "6"],
        svec!["b", "24", "12"],
        svec!["c", "72", "36"],
        svec!["d", "7", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_require_luadate() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number", "date_col"],
            svec!["a", "13", "2001-09-11"],
            svec!["b", "24", "1989-11-09"],
            svec!["c", "72", "2008-11-04"],
            svec!["d", "7", "2020-03-11"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("days_added")
        .arg(r#"local date = require "date";local t_date = date(date_col):adddays(tonumber(number)); return tostring(t_date:fmt("${iso}"))"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "date_col", "days_added"],
        svec!["a", "13", "2001-09-11", "2001-09-24T00:00:00"],
        svec!["b", "24", "1989-11-09", "1989-12-03T00:00:00"],
        svec!["c", "72", "2008-11-04", "2009-01-15T00:00:00"],
        svec!["d", "7", "2020-03-11", "2020-03-18T00:00:00"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_require() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number1", "number2"],
            svec!["a", "13", "43"],
            svec!["b", "24", "3.14"],
            svec!["c", "72", "42"],
            svec!["d", "7", "6.5"],
        ],
    );

    let mintest_luau = r#"
local mintest = {}

function mintest.mymin(n1, n2)
    if (tonumber(n1) < tonumber(n2)) then
        return n1;
    else
        return n2;
    end
end

return mintest"#;

    wrk.create_from_string("mintest.luau", mintest_luau);

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("min")
        .arg(r#"local mintest = require "mintest";local t_min = mintest.mymin(number1,number2); return t_min"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number1", "number2", "min"],
        svec!["a", "13", "43", "13"],
        svec!["b", "24", "3.14", "3.14"],
        svec!["c", "72", "42", "42"],
        svec!["d", "7", "6.5", "6.5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_error() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("div")
        .arg("math.dancefloor(number / 2)")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
    let stderr_string = wrk.output_stderr(&mut cmd);
    assert!(stderr_string.ends_with("Luau errors encountered: 4\n"));
}

#[test]
fn luau_map_header_with_nonalphanumeric_chars() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter-column", "number column"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg(r#"div/column"#)
        .arg(r#"col["letter-column"] .. math.floor(col["number column"] / 2)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter-column", "number column", "div/column"],
        svec!["a", "13", "a6"],
        svec!["b", "24", "b12"],
        svec!["c", "72", "c36"],
        svec!["d", "7", "d3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_no_headers() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("col[2] + 1")
        .arg("--no-headers")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_no_headers_multiple_new_columns() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("{col[2] + 1, col[2] + 2, col[2] + 3}")
        .arg("--no-headers")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "13", "14", "15", "16"],
        svec!["b", "24", "25", "26", "27"],
        svec!["c", "72", "73", "74", "75"],
        svec!["d", "7", "8", "9", "10"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_exec() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "x"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("running_total")
        .arg("tot = (tot or 0) + x; return tot")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "x", "running_total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_exec_long_column_names() {
    let wrk = Workdir::new("luau_map_exec_long_column_names");
    wrk.create(
        "data.csv",
        vec![
            svec!["the letter column", "the number column"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("running_total")
        .arg("tot = (tot or 0) + col['the number column']; return tot")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["the letter column", "the number column", "running_total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_no_globals() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["y", "x"],
            svec!["1", "13"],
            svec!["2", "24"],
            svec!["3", "72"],
            svec!["4", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("z")
        .arg("-g")
        .arg("--colindex")
        .arg("(x or col[1]) + 1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["y", "x", "z"],
        svec!["1", "13", "2"],
        svec!["2", "24", "3"],
        svec!["3", "72", "4"],
        svec!["4", "7", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_boolean() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("test")
        .arg("tonumber(number) > 14")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "test"],
        svec!["a", "13", "false"],
        svec!["b", "24", "true"],
        svec!["c", "72", "true"],
        svec!["d", "7", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_filter() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("filter")
        .arg("tonumber(number) > 14")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["b", "24"],
        svec!["c", "72"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_filter_error() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("filter")
        .arg("tothedancenumber(number) > 14")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["d", "7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_filter_num() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "-7"],
            svec!["e", "0"],
            svec!["f", "42"],
            svec!["g", "0.0"],
            svec!["h", "3.14"],
            svec!["i", "0.000123"],
            svec!["j", "-7.01"],
            svec!["k", "0.0000"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("filter").arg("tonumber(number)").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["d", "-7"],
        svec!["f", "42"],
        svec!["h", "3.14"],
        svec!["i", "0.000123"],
        svec!["j", "-7.01"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_envvars() {
    let wrk = Workdir::new("luau_envvars");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "-7"],
            svec!["e", "0"],
            svec!["f", "42"],
            svec!["g", "0.0"],
            svec!["h", "3.14"],
            svec!["i", "0.000123"],
            svec!["j", "-7.01"],
            svec!["k", "0.0000"],
        ],
    );

    wrk.create_from_string(
        "testenvvar.luau",
        r#"
BEGIN {
    qsv_setenv("TESTENVVAR", "11");
}!

local limit = qsv_getenv("TESTENVVAR")
return tonumber(number) > tonumber(limit)
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("filter")
        .arg("file:testenvvar.luau")
        .arg("data.csv");

    let testenvvar = std::env::var("TESTENVVAR").unwrap_or_else(|_| "NOT SET".to_string());
    assert_eq!(testenvvar, "NOT SET");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["f", "42"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_envvars_external() {
    let wrk = Workdir::new("luau_envvars_external");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "-7"],
            svec!["e", "0"],
            svec!["f", "42"],
            svec!["g", "0.0"],
            svec!["h", "3.14"],
            svec!["i", "0.000123"],
            svec!["j", "-7.01"],
            svec!["k", "0.0000"],
        ],
    );

    wrk.create_from_string(
        "testenvvar.luau",
        r#"
local limit = qsv_getenv("TESTENVVAR")
return tonumber(number) > tonumber(limit)
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("filter")
        .arg("file:testenvvar.luau")
        .arg("data.csv")
        .env("TESTENVVAR", "10");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["f", "42"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_loadjson() {
    let wrk = Workdir::new("luau_loadjson");

    // Create a test JSON file
    wrk.create_from_string(
        "test.json",
        r#"{
            "string": "hello",
            "number": 42,
            "boolean": true,
            "null": null,
            "array": [1, 2, 3],
            "object": {
                "nested": "value",
                "numbers": [4, 5, 6]
            }
        }"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("result")
        .arg(r#"
BEGIN {
    qsv_loadjson("config_tbl", "test.json");
    qsv_log("debug", "Loaded JSON table:", config_tbl)
    for k,v in pairs(config_tbl) do
        qsv_log("debug", "Key:", k, "Value:", v)
    end
}!

return config_tbl["string"] .. " " .. config_tbl["object"]["nested"] .. " " .. tostring(config_tbl["number"])
"#)
        .arg("--no-headers")
        .arg("data.csv");

    // Create a simple input CSV
    wrk.create("data.csv", vec![svec!["a"], svec!["b"], svec!["c"]]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "hello value 42"],
        svec!["b", "hello value 42"],
        svec!["c", "hello value 42"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_loadjson_array() {
    let wrk = Workdir::new("luau_loadjson_array");

    // Create a test JSON file with an array
    wrk.create_from_string(
        "test.json",
        r#"[
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"},
            {"id": 3, "name": "Charlie"}
        ]"#,
    );

    // Create input CSV with index references
    wrk.create(
        "data.csv",
        vec![svec!["x"], svec!["1"], svec!["2"], svec!["3"]],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("name")
        .arg(
            r#"
BEGIN {
    qsv_loadjson("users", "test.json");
}!

local idx = tonumber(x)
return users[idx]["name"]
"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["x", "name"],
        svec!["1", "Alice"],
        svec!["2", "Bob"],
        svec!["3", "Charlie"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_loadjson_error_missing_file() {
    let wrk = Workdir::new("luau_loadjson_error");

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("result")
        .arg(
            r#"
BEGIN {
    qsv_loadjson("config", "nonexistent.json");
}!

return "should not get here"
"#,
        )
        .arg("--no-headers")
        .arg("data.csv");

    // Create a simple input CSV
    wrk.create("data.csv", vec![svec!["a"]]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn luau_loadjson_error_invalid_json() {
    let wrk = Workdir::new("luau_loadjson_invalid");

    // Create an invalid JSON file
    wrk.create_from_string(
        "invalid.json",
        r#"{
            "unclosed": "object"
        "#, // Missing closing brace
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("result")
        .arg(
            r#"
BEGIN {
    qsv_loadjson("config", "invalid.json");
}!

return "should not get here"
"#,
        )
        .arg("--no-headers")
        .arg("data.csv");

    // Create a simple input CSV
    wrk.create("data.csv", vec![svec!["a"]]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn luau_cumsum_single() {
    let wrk = Workdir::new("luau_cumsum_single");
    wrk.create(
        "data.csv",
        vec![
            svec!["amount"],
            svec!["10"],
            svec!["20"],
            svec!["30"],
            svec!["40"],
            svec!["50"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("running_total")
        .arg(r#"qsv_cumsum(amount)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["amount", "running_total"],
        svec!["10", "10"],
        svec!["20", "30"],
        svec!["30", "60"],
        svec!["40", "100"],
        svec!["50", "150"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumsum_multiple() {
    let wrk = Workdir::new("luau_cumsum_multiple");
    wrk.create(
        "data.csv",
        vec![
            svec!["revenue", "expenses"],
            svec!["100", "50"],
            svec!["200", "75"],
            svec!["150", "80"],
            svec!["300", "100"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("revenue_total,expenses_total,profit_total")
        .arg(
            r#"{
            qsv_cumsum(revenue, "revenue"),
            qsv_cumsum(expenses, "expenses"),
            qsv_cumsum(tonumber(revenue) - tonumber(expenses), "profit")
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "revenue",
            "expenses",
            "revenue_total",
            "expenses_total",
            "profit_total"
        ],
        svec!["100", "50", "100", "50", "50"],
        svec!["200", "75", "300", "125", "175"],
        svec!["150", "80", "450", "205", "245"],
        svec!["300", "100", "750", "305", "445"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumsum_invalid_input() {
    let wrk = Workdir::new("luau_cumsum_invalid");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["invalid"],
            svec!["20"],
            svec!["bad"],
            svec!["30"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("total")
        .arg(r#"qsv_cumsum(value)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "total"],
        svec!["10", "10"],
        svec!["invalid", "10"], // Invalid input treated as 0
        svec!["20", "30"],
        svec!["bad", "30"], // Invalid input treated as 0
        svec!["30", "60"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumprod() {
    let wrk = Workdir::new("luau_cumprod");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["2"],
            svec!["3"],
            svec!["4"],
            svec!["5"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("running_product")
        .arg(r#"qsv_cumprod(value)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "running_product"],
        svec!["2", "2"],
        svec!["3", "6"],
        svec!["4", "24"],
        svec!["5", "120"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cummax_cummin() {
    let wrk = Workdir::new("luau_cummax_cummin");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["5"],
            svec!["15"],
            svec!["3"],
            svec!["12"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("running_max,running_min")
        .arg(r#"{qsv_cummax(value), qsv_cummin(value)}"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "running_max", "running_min"],
        svec!["10", "10", "10"],
        svec!["5", "10", "5"],
        svec!["15", "15", "5"],
        svec!["3", "15", "3"],
        svec!["12", "15", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_lag() {
    let wrk = Workdir::new("luau_lag");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["20"],
            svec!["30"],
            svec!["40"],
            svec!["50"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("lag1,lag2")
        .arg(
            r#"{
            qsv_lag(value, 1, "0"),
            qsv_lag(value, 2, "0")
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "lag1", "lag2"],
        svec!["10", "0", "0"],
        svec!["20", "10", "0"],
        svec!["30", "20", "10"],
        svec!["40", "30", "20"],
        svec!["50", "40", "30"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumany_cumall() {
    let wrk = Workdir::new("luau_cumany_cumall");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["0"],
            svec!["15"],
            svec!["0"],
            svec!["20"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("has_zero,all_positive")
        .arg(
            r#"{
            qsv_cumany(value == "0"),
            qsv_cumall(tonumber(value) > 0)
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "has_zero", "all_positive"],
        svec!["10", "false", "true"],
        svec!["0", "true", "false"],
        svec!["15", "true", "false"],
        svec!["0", "true", "false"],
        svec!["20", "true", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_diff() {
    let wrk = Workdir::new("luau_diff");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["15"],
            svec!["13"],
            svec!["20"],
            svec!["18"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("diff1,diff2")
        .arg(
            r#"{
            qsv_diff(value),
            qsv_diff(value, 2)
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "diff1", "diff2"],
        svec!["10", "0", "0"],
        svec!["15", "5", "0"],
        svec!["13", "-2", "3"],
        svec!["20", "7", "5"],
        svec!["18", "-2", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumany_cumall_with_strings() {
    let wrk = Workdir::new("luau_cumany_cumall_strings");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["hello"],
            svec![""],
            svec!["world"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("has_empty,all_nonempty")
        .arg(
            r#"{
            qsv_cumany(value == ""),
            qsv_cumall(value ~= "")
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "has_empty", "all_nonempty"],
        svec!["hello", "false", "true"],
        svec!["", "true", "false"],
        svec!["world", "true", "false"],
        svec!["", "true", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumall_with_numbers() {
    let wrk = Workdir::new("luau_cumall_numbers");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["0"],
            svec!["20"],
            svec!["30"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("all_positive")
        .arg(r#"qsv_cumall(tonumber(value) > 0)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "all_positive"],
        svec!["10", "true"],
        svec!["0", "false"],
        svec!["20", "false"],
        svec!["30", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_lag_with_default() {
    let wrk = Workdir::new("luau_lag_default");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["20"],
            svec!["30"],
            svec!["40"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("prev,prev2")
        .arg(
            r#"{
            qsv_lag(value, 1, "N/A"),
            qsv_lag(value, 2, "N/A")
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "prev", "prev2"],
        svec!["10", "N/A", "N/A"],
        svec!["20", "10", "N/A"],
        svec!["30", "20", "10"],
        svec!["40", "30", "20"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_diff_with_lag() {
    let wrk = Workdir::new("luau_diff_lag");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["100"],
            svec!["120"],
            svec!["115"],
            svec!["140"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("diff1,diff2,pct_change")
        .arg(
            r#"{
            qsv_diff(value),
            qsv_diff(value, 2),
            string.format("%.1f%%", ((tonumber(value) / tonumber(qsv_lag(value))) - 1.0) * 100)
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "diff1", "diff2", "pct_change"],
        svec!["100", "0", "0", "inf%"], // division by zero
        svec!["120", "20", "0", "20.0%"],
        svec!["115", "-5", "15", "-4.2%"],
        svec!["140", "25", "20", "21.7%"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumulative_stats() {
    let wrk = Workdir::new("luau_cumulative_stats");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["15"],
            svec!["10"],
            svec!["25"],
            svec!["5"],
            svec!["20"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("running_max,running_min,running_sum")
        .arg(
            r#"{
            qsv_cummax(value),
            qsv_cummin(value),
            qsv_cumsum(value)
        }"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "running_max", "running_min", "running_sum"],
        svec!["15", "15", "15", "15"],
        svec!["10", "15", "10", "25"],
        svec!["25", "25", "10", "50"],
        svec!["5", "25", "5", "55"],
        svec!["20", "25", "5", "75"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_writejson_simple() {
    let wrk = Workdir::new("luau_writejson");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    // Test writing a simple table to JSON
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("result")
        .arg(
            r#"
BEGIN {
    -- Create a table to store
    local data = {
        numbers = {13, 24, 72, 7},
        letters = {"a", "b", "c", "d"},
        nested = {
            key = "value",
            array = {1, 2, 3}
        }
    }
    -- Write both pretty and compact JSON
    qsv_writejson(data, "output.json", false)
    qsv_writejson(data, "output_pretty.json", true)
}!

return "ok"
"#,
        )
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    // Verify the compact JSON output
    let json_content = std::fs::read_to_string(wrk.path("output.json")).unwrap();
    assert_eq!(
        json_content,
        r#"{"numbers":[13,24,72,7],"nested":{"key":"value","array":[1,2,3]},"letters":["a","b","c","d"]}"#
    );

    // Verify the pretty JSON output has newlines and indentation
    let pretty_content = std::fs::read_to_string(wrk.path("output_pretty.json")).unwrap();
    let expected = r#"{
  "numbers": [
    13,
    24,
    72,
    7
  ],
  "nested": {
    "key": "value",
    "array": [
      1,
      2,
      3
    ]
  },
  "letters": [
    "a",
    "b",
    "c",
    "d"
  ]
}"#;

    assert_eq!(pretty_content, expected);
}

#[test]
fn luau_writejson_special_values() {
    let wrk = Workdir::new("luau_writejson_special");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "13"]],
    );

    // Test writing various Lua types
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("result")
        .arg(
            r#"
BEGIN {
    local data = {
        null_value = nil,
        boolean_true = true,
        boolean_false = false,
        number_integer = 42,
        number_float = 3.14,
        empty_string = "",
        empty_table = {},
        nested_empty = {empty = {}}
    }
    qsv_writejson(data, "special.json", true)
}!

return "ok"
"#,
        )
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    // Verify the JSON output
    let json_content = std::fs::read_to_string(wrk.path("special.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap();

    assert!(json["null_value"].is_null());
    assert_eq!(json["boolean_true"], json!(true));
    assert_eq!(json["boolean_false"], json!(false));
    assert_eq!(json["number_integer"], json!(42));
    assert_eq!(json["number_float"], json!(3.14));
    assert_eq!(json["empty_string"], json!(""));
    assert_eq!(json["empty_table"], json!({}));
    assert_eq!(json["nested_empty"]["empty"], json!({}));
}

#[test]
fn luau_accumulate_custom_sum_function() {
    let wrk = Workdir::new("luau_accumulate_custom_sum_function");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["20"],
            svec!["30"],
            svec!["40"],
            svec!["50"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("accumulated")
        .arg(
            r#"
-- This is the MAIN LOOP
-- Define a custom accumulator function that keeps a weighted sum
-- where each new value is weighted by its position
-- we define the function inside the MAIN LOOP to access the _IDX variable
function weighted_sum(acc, x)
        return acc + x * _IDX
end

return qsv_accumulate(value, weighted_sum, 0)
"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "accumulated"],
        svec!["10", "10"],  // 10 + 0 * 1
        svec!["20", "50"],  // 10 + 20 * 2
        svec!["30", "140"], // 50 + 30 * 3
        svec!["40", "300"], // 140 + 40 * 4
        svec!["50", "550"], // 300 + 50 * 5
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_accumulate_custom_sum_function_no_initial_value() {
    let wrk = Workdir::new("luau_accumulate_custom_sum_function_no_initial_value");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["20"],
            svec!["30"],
            svec!["40"],
            svec!["50"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("accumulated")
        .arg(
            r#"
-- This is the MAIN LOOP
-- Define a custom accumulator function that keeps a weighted sum
-- where each new value is weighted by its position
-- we define the function inside the MAIN LOOP to access the _IDX variable
function weighted_sum(acc, x)
        return acc + x * _IDX
end

return qsv_accumulate(value, weighted_sum)
"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "accumulated"],
        svec!["10", "20"],  // 10 + 10 * 1
        svec!["20", "60"],  // 20 + 20 * 2
        svec!["30", "150"], // 60 + 30 * 3
        svec!["40", "310"], // 150 + 40 * 4
        svec!["50", "560"], // 310 + 50 * 5
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_accumulate_custom_sum_function_with_initial_value() {
    let wrk = Workdir::new("luau_accumulate_custom_sum_function_with_initial_value");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["10"],
            svec!["20"],
            svec!["30"],
            svec!["40"],
            svec!["50"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("accumulated")
        .arg(
            r#"
BEGIN {

    -- Define a custom accumulator function
    function udf_sum(acc, x)
         return acc + x
    end
}!

-- This is the MAIN LOOP
accumulated = qsv_accumulate(value, udf_sum, 100)

-- return the accumulated value for the current row
return accumulated
"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "accumulated"],
        svec!["10", "110"], // initial value
        svec!["20", "130"], // 110 + 20
        svec!["30", "160"], // 130 + 30
        svec!["40", "200"], // 160 + 40
        svec!["50", "250"], // 200 + 50
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_accumulate_custom_function_with_reset() {
    let wrk = Workdir::new("luau_accumulate_custom_with_reset");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["3"],
            svec!["5"],
            svec!["4"],
            svec!["1"],
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("accumulated")
        .arg(
            r#"
BEGIN {

    -- Define a custom accumulator function that resets the accumulator
    -- if the accumulated value exceeds a threshold of 7
    function func_with_reset(acc, x)
         if tonumber(acc) > 7 then
            z = x
         else
            z = acc + x
         end
         return z
    end
}!

-- This is the MAIN LOOP
return qsv_accumulate(value, func_with_reset, 0)
"#,
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["value", "accumulated"],
        svec!["3", "3"], // initial value
        svec!["5", "8"], // 3 + 5
        svec!["4", "4"], // reset since 8 > 7
        svec!["1", "5"], // 4 + 1
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_cumsum_overflow() {
    let wrk = Workdir::new("luau_cumsum_overflow");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["1e308"], // Very large number
            svec!["1e308"], // Adding these will overflow
        ],
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("total")
        .arg(r#"qsv_cumsum(value)"#)
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    let expected = "Luau errors encountered: 1\n";
    assert_eq!(stderr, expected);
}

#[test]
fn luau_shellcmd_invalid_command() {
    let wrk = Workdir::new("luau_shellcmd_invalid");
    wrk.create("data.csv", vec![svec!["value"], svec!["10"]]);

    // Test with an unauthorized command
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("result")
        .arg(
            r#"
BEGIN {
    local result = qsv_shellcmd("rm", "-rf /")  -- This command should not be allowed
}!
return "ok"
"#,
        )
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(stderr.contains("Invalid shell command"));
}
