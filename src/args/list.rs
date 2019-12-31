use crate::args::prelude::{str_to_location, Config};
use crate::args::types::{
    AppError, Command, CustomHoliday, DailyStudy, Event, Language, ListArgs, MinorHoliday,
    RambamChapters, YearType,
};
use clap::ArgMatches;
use heca_lib::prelude::{Location, TorahReadingType};
use std::env;

pub fn parse_options(
    matches: &ArgMatches<'_>,
    config: &Config,
    language: Language,
    custom_days: &[CustomHoliday],
) -> Result<Command, AppError> {
    let year_num = matches
        .value_of("Year")
        .unwrap()
        .parse()
        .expect("The supplied year must be a number");
    let amnt_years = matches
        .value_of("AmountYears")
        .unwrap()
        .parse()
        .expect("Amount of years must be a number");

    let year = match matches.value_of("YearType").unwrap() {
        "hebrew" => YearType::Hebrew(year_num),
        "gregorian" => YearType::Gregorian(year_num),
        "fuzzy" => {
            if year_num > 3000 {
                YearType::Hebrew(year_num)
            } else {
                YearType::Gregorian(year_num)
            }
        }
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let no_sort = matches.occurrences_of("NoSort") > 0;

    let location = if let Some(location) = matches.value_of("Location") {
        str_to_location(location)?
    } else if let Some(location) = env::var_os("LOC") {
        let location: String = location.into_string().unwrap();
        str_to_location(&location)?
    } else if let Some(location) = &config.location {
        *location
    } else if language == Language::Hebrew {
        Location::Israel
    } else {
        Location::Chul
    };

    let events = matches
        .values_of("Events")
        .unwrap_or_else(|| panic!("{}, {}", file!(), line!()))
        .flat_map(|x| match x {
            "yom-tov" => vec![Event::TorahReadingType(TorahReadingType::YomTov)],
            "chol" => vec![Event::TorahReadingType(TorahReadingType::Chol)],
            "shabbos" => vec![Event::TorahReadingType(TorahReadingType::Shabbos)],
            "special-parshas" => vec![Event::TorahReadingType(TorahReadingType::SpecialParsha)],
            "omer" => vec![Event::MinorHoliday(MinorHoliday::Omer)],
            "custom-holidays" => custom_days
                .iter()
                .map(|x| Event::CustomHoliday(x.clone()))
                .collect(),
            "minor-holidays" => vec![Event::MinorHoliday(MinorHoliday::Minor)],
            "daf-yomi" => vec![Event::DailyStudy(DailyStudy::DafYomi)],
            "yerushalmi-yomi" => vec![Event::DailyStudy(DailyStudy::YerushalmiYomi)],
            "rambam-3-chapters" => {
                vec![Event::DailyStudy(DailyStudy::Rambam(RambamChapters::Three))]
            }
            "rambam-1-chapter" => vec![Event::DailyStudy(DailyStudy::Rambam(RambamChapters::One))],
            x => unreachable!("{}", x),
        })
        .collect::<Vec<Event>>();
    Ok(Command::List(ListArgs {
        year,
        location,
        events,
        amnt_years,
        no_sort,
    }))
}