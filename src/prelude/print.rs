use crate::args::types;
use crate::args::types::{Language, MinorDays};
use heca_lib::prelude::{Chol, HebrewMonth, Parsha, SpecialParsha, TorahReading, YomTov};

pub fn minor_holidays(tr: MinorDays, language: types::Language) -> &'static str {
    match language {
        Language::English => match tr {
            //Generated from https://play.golang.org/p/HtWEMOgflMt
            MinorDays::Omer1 => "1st day of the Omer",
            MinorDays::Omer2 => "2nd day of the Omer",
            MinorDays::Omer3 => "3rd day of the Omer",
            MinorDays::Omer4 => "4th day of the Omer",
            MinorDays::Omer5 => "5th day of the Omer",
            MinorDays::Omer6 => "6th day of the Omer",
            MinorDays::Omer7 => "7th day of the Omer",
            MinorDays::Omer8 => "8th day of the Omer",
            MinorDays::Omer9 => "9th day of the Omer",
            MinorDays::Omer10 => "10th day of the Omer",
            MinorDays::Omer11 => "11th day of the Omer",
            MinorDays::Omer12 => "12th day of the Omer",
            MinorDays::Omer13 => "13th day of the Omer",
            MinorDays::Omer14 => "14th day of the Omer",
            MinorDays::Omer15 => "15th day of the Omer",
            MinorDays::Omer16 => "16th day of the Omer",
            MinorDays::Omer17 => "17th day of the Omer",
            MinorDays::Omer18 => "18th day of the Omer",
            MinorDays::Omer19 => "19th day of the Omer",
            MinorDays::Omer20 => "20th day of the Omer",
            MinorDays::Omer21 => "21st day of the Omer",
            MinorDays::Omer22 => "22nd day of the Omer",
            MinorDays::Omer23 => "23rd day of the Omer",
            MinorDays::Omer24 => "24th day of the Omer",
            MinorDays::Omer25 => "25th day of the Omer",
            MinorDays::Omer26 => "26th day of the Omer",
            MinorDays::Omer27 => "27th day of the Omer",
            MinorDays::Omer28 => "28th day of the Omer",
            MinorDays::Omer29 => "29th day of the Omer",
            MinorDays::Omer30 => "30th day of the Omer",
            MinorDays::Omer31 => "31st day of the Omer",
            MinorDays::Omer32 => "32nd day of the Omer",
            MinorDays::Omer33 => "33rd day of the Omer",
            MinorDays::Omer34 => "34th day of the Omer",
            MinorDays::Omer35 => "35th day of the Omer",
            MinorDays::Omer36 => "36th day of the Omer",
            MinorDays::Omer37 => "37th day of the Omer",
            MinorDays::Omer38 => "38th day of the Omer",
            MinorDays::Omer39 => "39th day of the Omer",
            MinorDays::Omer40 => "40th day of the Omer",
            MinorDays::Omer41 => "41st day of the Omer",
            MinorDays::Omer42 => "42nd day of the Omer",
            MinorDays::Omer43 => "43rd day of the Omer",
            MinorDays::Omer44 => "44th day of the Omer",
            MinorDays::Omer45 => "45th day of the Omer",
            MinorDays::Omer46 => "46th day of the Omer",
            MinorDays::Omer47 => "47th day of the Omer",
            MinorDays::Omer48 => "48th day of the Omer",
            MinorDays::Omer49 => "49th day of the Omer",
            MinorDays::ErevPesach => "Erev Pesach",
            MinorDays::ErevSukkos => "Erev Sukkos",
            MinorDays::ErevShavuos => "Erev Shavuos",
            MinorDays::ErevYomKippur => "Erev Yom Kippur",
            MinorDays::ErevRoshHashanah => "Erev Rosh Hashana",
            MinorDays::PesachSheni => "Pesach Sheni",
            MinorDays::LagBaOmer => "Lag BaOmer",
            MinorDays::FifteenAv => "15th of Av",
            MinorDays::FifteenShvat => "15th of Shevat",
            MinorDays::PurimKattan => "Purim Kattan",
            MinorDays::ShushanPurimKattan => "Shushan Purim Kattan",
            MinorDays::ShabbosHaGadol => "Shabbos HaGadol",
        },
        Language::Hebrew => match tr {
            //generated from https://play.golang.org/p/LH0qQmYxZsP
            MinorDays::Omer1 => "היום יום 1 לעומר",
            MinorDays::Omer2 => "היום יום 2 לעומר",
            MinorDays::Omer3 => "היום יום 3 לעומר",
            MinorDays::Omer4 => "היום יום 4 לעומר",
            MinorDays::Omer5 => "היום יום 5 לעומר",
            MinorDays::Omer6 => "היום יום 6 לעומר",
            MinorDays::Omer7 => "היום יום 7 לעומר",
            MinorDays::Omer8 => "היום יום 8 לעומר",
            MinorDays::Omer9 => "היום יום 9 לעומר",
            MinorDays::Omer10 => "היום יום 10 לעומר",
            MinorDays::Omer11 => "היום יום 11 לעומר",
            MinorDays::Omer12 => "היום יום 12 לעומר",
            MinorDays::Omer13 => "היום יום 13 לעומר",
            MinorDays::Omer14 => "היום יום 14 לעומר",
            MinorDays::Omer15 => "היום יום 15 לעומר",
            MinorDays::Omer16 => "היום יום 16 לעומר",
            MinorDays::Omer17 => "היום יום 17 לעומר",
            MinorDays::Omer18 => "היום יום 18 לעומר",
            MinorDays::Omer19 => "היום יום 19 לעומר",
            MinorDays::Omer20 => "היום יום 20 לעומר",
            MinorDays::Omer21 => "היום יום 21 לעומר",
            MinorDays::Omer22 => "היום יום 22 לעומר",
            MinorDays::Omer23 => "היום יום 23 לעומר",
            MinorDays::Omer24 => "היום יום 24 לעומר",
            MinorDays::Omer25 => "היום יום 25 לעומר",
            MinorDays::Omer26 => "היום יום 26 לעומר",
            MinorDays::Omer27 => "היום יום 27 לעומר",
            MinorDays::Omer28 => "היום יום 28 לעומר",
            MinorDays::Omer29 => "היום יום 29 לעומר",
            MinorDays::Omer30 => "היום יום 30 לעומר",
            MinorDays::Omer31 => "היום יום 31 לעומר",
            MinorDays::Omer32 => "היום יום 32 לעומר",
            MinorDays::Omer33 => "היום יום 33 לעומר",
            MinorDays::Omer34 => "היום יום 34 לעומר",
            MinorDays::Omer35 => "היום יום 35 לעומר",
            MinorDays::Omer36 => "היום יום 36 לעומר",
            MinorDays::Omer37 => "היום יום 37 לעומר",
            MinorDays::Omer38 => "היום יום 38 לעומר",
            MinorDays::Omer39 => "היום יום 39 לעומר",
            MinorDays::Omer40 => "היום יום 40 לעומר",
            MinorDays::Omer41 => "היום יום 41 לעומר",
            MinorDays::Omer42 => "היום יום 42 לעומר",
            MinorDays::Omer43 => "היום יום 43 לעומר",
            MinorDays::Omer44 => "היום יום 44 לעומר",
            MinorDays::Omer45 => "היום יום 45 לעומר",
            MinorDays::Omer46 => "היום יום 46 לעומר",
            MinorDays::Omer47 => "היום יום 47 לעומר",
            MinorDays::Omer48 => "היום יום 48 לעומר",
            MinorDays::Omer49 => "היום יום 49 לעומר",
            MinorDays::ErevPesach => "ערב פסח",
            MinorDays::ErevSukkos => "ערב סוכות",
            MinorDays::ErevShavuos => "ערב שבועות",
            MinorDays::ErevYomKippur => "ערב יום כיפור",
            MinorDays::ErevRoshHashanah => "ערב ראש השנה",
            MinorDays::PesachSheni => "ערב פסח שני",
            MinorDays::LagBaOmer => "ל\"ג בעומר",
            MinorDays::FifteenAv => "ט\"ו באב",
            MinorDays::FifteenShvat => "ט\"ו בשבט",
            MinorDays::PurimKattan => "פורים קטן",
            MinorDays::ShushanPurimKattan => "שושן פורים קטן",
            MinorDays::ShabbosHaGadol => "שבת הגדול",
        },
    }
}

pub fn hebrew_month_hebrew(h: HebrewMonth) -> &'static str {
    match h {
        HebrewMonth::Tishrei => "תשרי",
        HebrewMonth::Cheshvan => "חשוון",
        HebrewMonth::Kislev => "כסלו",
        HebrewMonth::Teves => "טבת",
        HebrewMonth::Shvat => "שבט",
        HebrewMonth::Adar => "אדר",
        HebrewMonth::Adar1 => "אדר א",
        HebrewMonth::Adar2 => "אדר ב",
        HebrewMonth::Nissan => "ניסן",
        HebrewMonth::Iyar => "אייר",
        HebrewMonth::Sivan => "סיוון",
        HebrewMonth::Tammuz => "תמוז",
        HebrewMonth::Av => "אב",
        HebrewMonth::Elul => "אלול",
    }
}

pub fn hebrew_month_english(h: HebrewMonth) -> &'static str {
    match h {
        HebrewMonth::Tishrei => "Tishrei",
        HebrewMonth::Cheshvan => "Cheshvan",
        HebrewMonth::Kislev => "Kislev",
        HebrewMonth::Teves => "Teves",
        HebrewMonth::Shvat => "Shvat",
        HebrewMonth::Adar => "Adar",
        HebrewMonth::Adar1 => "Adar Rishon",
        HebrewMonth::Adar2 => "Adar Sheni",
        HebrewMonth::Nissan => "Nissan",
        HebrewMonth::Iyar => "Iyar",
        HebrewMonth::Sivan => "Sivan",
        HebrewMonth::Tammuz => "Tammuz",
        HebrewMonth::Av => "Av",
        HebrewMonth::Elul => "Elul",
    }
}

pub fn torah_reading(tr: TorahReading, language: types::Language) -> &'static str {
    match language {
        Language::English => match tr {
            TorahReading::YomTov(yt) => match yt {
                YomTov::RoshHashanah1 => "1st day of Rosh Hashanah",
                YomTov::RoshHashanah2 => "2nd day of Rosh Hashanah",
                YomTov::YomKippur => "Yom Kippur",
                YomTov::Sukkos1 => "1st day of Sukkos",
                YomTov::Sukkos2 => "2nd day of Sukkos",
                YomTov::Sukkos3 => "3rd day of Sukkos",
                YomTov::Sukkos4 => "4th day of Sukkos",
                YomTov::Sukkos5 => "5th day of Sukkos",
                YomTov::Sukkos6 => "6th day of Sukkos",
                YomTov::Sukkos7 => "7th day of Sukkos",
                YomTov::ShminiAtzeres => "Shmini Atzeres",
                YomTov::SimchasTorah => "Simchas Torah",
                YomTov::Pesach1 => "1st day of Pesach",
                YomTov::Pesach2 => "2nd day of Pesach",
                YomTov::Pesach3 => "3rd day of Pesach",
                YomTov::Pesach4 => "4th day of Pesach",
                YomTov::Pesach5 => "5th day of Pesach",
                YomTov::Pesach6 => "6th day of Pesach",
                YomTov::Pesach7 => "7th day of Pesach",
                YomTov::Pesach8 => "8th day of Pesach",
                YomTov::Shavuos1 => "1st day of Shavuos",
                YomTov::Shavuos2 => "2nd day of Shavuos",
            },
            TorahReading::Chol(tr) => match tr {
                Chol::RoshChodeshCheshvan1 => "1st day of Rosh Chodesh Cheshvan",
                Chol::RoshChodeshCheshvan2 => "2nd day of Rosh Chodesh Cheshvan",
                Chol::RoshChodeshKislev => "Rosh Chodesh Kislev",
                Chol::RoshChodeshKislev1 => "1st day of Rosh Chodesh Kislev",
                Chol::RoshChodeshKislev2 => "2nd day of Rosh Chodesh Kislev",
                Chol::RoshChodeshTeves => "Rosh Chodesh Teves",
                Chol::RoshChodeshTeves1 => "1st day of Rosh Chodesh Teves",
                Chol::RoshChodeshTeves2 => "2nd day of Rosh Chodesh Teves",
                Chol::RoshChodeshShvat => "Rosh Chodesh Shvat",
                Chol::RoshChodeshAdar1 => "1st day of Rosh Chodesh Adar",
                Chol::RoshChodeshAdar2 => "2nd day of Rosh Chodesh Adar",
                Chol::RoshChodeshAdarRishon1 => "1st day of Rosh Chodesh Adar Rishon",
                Chol::RoshChodeshAdarRishon2 => "2nd day of Rosh Chodesh Adar Rishon",
                Chol::RoshChodeshAdarSheni1 => "1st day of Rosh Chodesh Adar Sheni",
                Chol::RoshChodeshAdarSheni2 => "2nd day of Rosh Chodesh Adar Sheni",
                Chol::RoshChodeshNissan => "Rosh Chodesh Nissan",
                Chol::RoshChodeshIyar1 => "1st day of Rosh Chodesh Iyar",
                Chol::RoshChodeshIyar2 => "2nd day of Rosh Chodesh Iyar",
                Chol::RoshChodeshSivan => "Rosh Chodesh Sivan",
                Chol::RoshChodeshTammuz1 => "1st day of Rosh Chodesh Tammuz",
                Chol::RoshChodeshTammuz2 => "2nd day of Rosh Chodesh Tammuz",
                Chol::RoshChodeshAv => "Rosh Chodesh Av",
                Chol::RoshChodeshElul1 => "1st day of Rosh Chodesh Elul",
                Chol::RoshChodeshElul2 => "2nd day of Rosh Chodesh Elul",
                Chol::Chanukah1 => "1st day of Chanukah",
                Chol::Chanukah2 => "2nd day of Chanukah",
                Chol::Chanukah3 => "3rd day of Chanukah",
                Chol::Chanukah4 => "4rd day of Chanukah",
                Chol::Chanukah5 => "5rd day of Chanukah",
                Chol::Chanukah6 => "6rd day of Chanukah",
                Chol::Chanukah7 => "7rd day of Chanukah",
                Chol::Chanukah8 => "8rd day of Chanukah",
                Chol::TzomGedalia => "Tzom Gedalia",
                Chol::TaanisEsther => "Taanis Esther",
                Chol::TenTeves => "Tenth of Teves",
                Chol::Purim => "Purim",
                Chol::ShushanPurim => "Shushan Purim",
                Chol::SeventeenTammuz => "Seventeenth of Tammuz",
                Chol::NineAv => "Ninth of Av",
            },
            TorahReading::Shabbos(tr) => match tr {
                Parsha::Haazinu => "Haazina",
                Parsha::Vayelech => "Vayelech",
                Parsha::Bereishis => "Bereishis",
                Parsha::Noach => "Noach",
                Parsha::LechLecha => "Lech Lecha",
                Parsha::Vayeira => "Vayeira",
                Parsha::ChayeiSara => "Chayei Sarah",
                Parsha::Toldos => "Toldos",
                Parsha::Vayetzei => "Vayetzei",
                Parsha::Vayishlach => "Vayishlach",
                Parsha::Vayeshev => "Vayeshev",
                Parsha::Miketz => "Miketz",
                Parsha::Vayigash => "Vayigash",
                Parsha::Vayechi => "Vayechi",
                Parsha::Shemos => "Shemos",
                Parsha::Vaeira => "Vaeira",
                Parsha::Bo => "Bo",
                Parsha::Beshalach => "Beshalach",
                Parsha::Yisro => "Yisro",
                Parsha::Mishpatim => "Mishpatim",
                Parsha::Terumah => "Terumah",
                Parsha::Tetzaveh => "Tetzaveh",
                Parsha::KiSisa => "Ki Sisa",
                Parsha::VayakhelPikudei => "Vayakhel/Pikudei",
                Parsha::Vayakhel => "Vayakhel",
                Parsha::Pikudei => "Pikudei",
                Parsha::Vayikra => "Vayikra",
                Parsha::Tzav => "Tzav",
                Parsha::Shemini => "Shemini",
                Parsha::TazriyaMetzorah => "Tazriya/Metzorah",
                Parsha::Tazriya => "Tazriya",
                Parsha::Metzorah => "Metzorah",
                Parsha::AchareiMosKedoshim => "Acharei Mos/Kedoshim",
                Parsha::AchareiMos => "Acharei Mos",
                Parsha::Kedoshim => "Kedoshim",
                Parsha::Emor => "Emor",
                Parsha::BeharBechukosai => "Behar/Bechukosai",
                Parsha::Behar => "Behar",
                Parsha::Bechukosai => "Bechukosai",
                Parsha::Bamidbar => "Bamidbar",
                Parsha::Naso => "Naso",
                Parsha::Behaaloscha => "Behaaloscha",
                Parsha::Shlach => "Shlach",
                Parsha::Korach => "Korach",
                Parsha::ChukasBalak => "Chukas/Balak",
                Parsha::Chukas => "Chukas",
                Parsha::Balak => "Balak",
                Parsha::Pinchas => "Pinchas",
                Parsha::MatosMaasei => "Matos/Maasei",
                Parsha::Matos => "Matos",
                Parsha::Maasei => "Maasei",
                Parsha::Devarim => "Devarim",
                Parsha::Vaeschanan => "Vaeschanan",
                Parsha::Eikev => "Eikev",
                Parsha::Reeh => "Re'eh",
                Parsha::Shoftim => "Shoftim",
                Parsha::KiSeitzei => "Ki Seitzei",
                Parsha::KiSavoh => "Ki Savo",
                Parsha::NitzavimVayelech => "Nitzavim/Vayelech",
                Parsha::Nitzavim => "Nitzavim",
            },
            TorahReading::SpecialParsha(tr) => match tr {
                SpecialParsha::Zachor => "Parshas Zachor",
                SpecialParsha::HaChodesh => "Parshas HaChodesh",
                SpecialParsha::Parah => "Parshas Parah",
                SpecialParsha::Shekalim => "Parshas Shekalim",
            },
        },
        Language::Hebrew => match tr {
            TorahReading::YomTov(yt) => match yt {
                YomTov::RoshHashanah1 => "יןם א של ראש השנה",
                YomTov::RoshHashanah2 => "יןם ב של ראש השנה",
                YomTov::YomKippur => "יום כיפור",
                YomTov::Sukkos1 => "יום א של חג הסוכות",
                YomTov::Sukkos2 => "יום ב של חג הסוכות",
                YomTov::Sukkos3 => "יום ג  של חג הסוכות",
                YomTov::Sukkos4 => "יום ד של חג הסוכות",
                YomTov::Sukkos5 => "יום ה של חג הסוכות",
                YomTov::Sukkos6 => "יום ו של חג הסוכות",
                YomTov::Sukkos7 => "יום ז של חג הסוכות",
                YomTov::ShminiAtzeres => "שמיני עצרת",
                YomTov::SimchasTorah => "שמחת תורה",
                YomTov::Pesach1 => "יום א של חג הפסח",
                YomTov::Pesach2 => "יום ב של חג הפסח",
                YomTov::Pesach3 => "יום ג של חג הפסח",
                YomTov::Pesach4 => "יום ד של חג הפסח",
                YomTov::Pesach5 => "יום ה של חג הפסח",
                YomTov::Pesach6 => "יום ו של חג הפסח",
                YomTov::Pesach7 => "יום ז של חג הפסח",
                YomTov::Pesach8 => "יום ח של חג הפסח",
                YomTov::Shavuos1 => "יום א של חג השבועות",
                YomTov::Shavuos2 => "יום ב של חג השבועות",
            },
            TorahReading::Chol(tr) => match tr {
                Chol::RoshChodeshCheshvan1 => "יום א של ראש חודש חשון",
                Chol::RoshChodeshCheshvan2 => "יום ב של ראש חודש חשון",
                Chol::RoshChodeshKislev => "ראש חודש כסלו",
                Chol::RoshChodeshKislev1 => "יום א של ראש חודש כסלו",
                Chol::RoshChodeshKislev2 => "יום ב של ראש חודש כסלו",
                Chol::RoshChodeshTeves => "ראש חודש טבת",
                Chol::RoshChodeshTeves1 => "יום א של ראש חודש טבת",
                Chol::RoshChodeshTeves2 => "יום ב של ראש חודש טבת",
                Chol::RoshChodeshShvat => "ראש חודש שבט",
                Chol::RoshChodeshAdar1 => "יום א של ראש חודש אדר",
                Chol::RoshChodeshAdar2 => "יום ב של ראש חודש אדר",
                Chol::RoshChodeshAdarRishon1 => "יום א של ראש חודש אדר ראשון",
                Chol::RoshChodeshAdarRishon2 => "יום ב של ראש חודש אדר ראשון",
                Chol::RoshChodeshAdarSheni1 => "יום א של ראש חודש אדר שני",
                Chol::RoshChodeshAdarSheni2 => "יום ב של ראש חודש אדר שני",
                Chol::RoshChodeshNissan => "ראש חדש ניסן",
                Chol::RoshChodeshIyar1 => "יום א של ראש חודש אייר",
                Chol::RoshChodeshIyar2 => "יום ב של ראש חודש אייר",
                Chol::RoshChodeshSivan => "ראש חדש סיון",
                Chol::RoshChodeshTammuz1 => "יום א של ראש חודש תמוז",
                Chol::RoshChodeshTammuz2 => "יום ב של ראש חודש תמוז",
                Chol::RoshChodeshAv => "ראש חודש אב",
                Chol::RoshChodeshElul1 => "יום א של ראש חודש אלול",
                Chol::RoshChodeshElul2 => "יום ב של ראש חודש אלול",
                Chol::Chanukah1 => "יום א של חנוכה",
                Chol::Chanukah2 => "יום ב של חנוכה",
                Chol::Chanukah3 => "יום ג של חנוכה",
                Chol::Chanukah4 => "יום ד של חנוכה",
                Chol::Chanukah5 => "יום ה של חנוכה",
                Chol::Chanukah6 => "יום ו של חנוכה",
                Chol::Chanukah7 => "יום ז של חנוכה",
                Chol::Chanukah8 => "יום ח של חנוכה",
                Chol::TzomGedalia => "צום גדליה",
                Chol::TaanisEsther => "תענית אסתר",
                Chol::TenTeves => "י' טבת",
                Chol::Purim => "פורים",
                Chol::ShushanPurim => "שושן פורים",
                Chol::SeventeenTammuz => "שבעה עשר בתמוז",
                Chol::NineAv => "תשעה באב",
            },
            TorahReading::Shabbos(tr) => match tr {
                Parsha::Haazinu => "האזינו",
                Parsha::Vayelech => "וילך",
                Parsha::Bereishis => "בראשית",
                Parsha::Noach => "נח",
                Parsha::LechLecha => "לך לך",
                Parsha::Vayeira => "וירא",
                Parsha::ChayeiSara => "חיי שרה",
                Parsha::Toldos => "תולדות",
                Parsha::Vayetzei => "ויצא",
                Parsha::Vayishlach => "וישלח",
                Parsha::Vayeshev => "וישב",
                Parsha::Miketz => "מיקץ",
                Parsha::Vayigash => "ויגש",
                Parsha::Vayechi => "ויחי",
                Parsha::Shemos => "שמות",
                Parsha::Vaeira => "וארא",
                Parsha::Bo => "בא",
                Parsha::Beshalach => "בשלח",
                Parsha::Yisro => "יתרו",
                Parsha::Mishpatim => "משפטים",
                Parsha::Terumah => "תרומה",
                Parsha::Tetzaveh => "תצוה",
                Parsha::KiSisa => "כי תשא",
                Parsha::VayakhelPikudei => "ויקהל/פקודי",
                Parsha::Vayakhel => "ויקהל",
                Parsha::Pikudei => "פקודי",
                Parsha::Vayikra => "ויקרא",
                Parsha::Tzav => "צו",
                Parsha::Shemini => "שמיני",
                Parsha::TazriyaMetzorah => "תזריע/מצורע",
                Parsha::Tazriya => "תזריע",
                Parsha::Metzorah => "מצורע",
                Parsha::AchareiMosKedoshim => "אחרי מות/קדושים",
                Parsha::AchareiMos => "אחרי מות",
                Parsha::Kedoshim => "קדושים",
                Parsha::Emor => "אמור",
                Parsha::BeharBechukosai => "בהר/בחוקותי",
                Parsha::Behar => "בהר",
                Parsha::Bechukosai => "בחוקותי",
                Parsha::Bamidbar => "במדבר",
                Parsha::Naso => "נשא",
                Parsha::Behaaloscha => "בהעלותך",
                Parsha::Shlach => "שלח",
                Parsha::Korach => "קרח",
                Parsha::ChukasBalak => "חקת/בלק",
                Parsha::Chukas => "חקת",
                Parsha::Balak => "בלק",
                Parsha::Pinchas => "פינחס",
                Parsha::MatosMaasei => "מטות/מסעי",
                Parsha::Matos => "מטות",
                Parsha::Maasei => "מסעי",
                Parsha::Devarim => "דברים",
                Parsha::Vaeschanan => "ואתחנן",
                Parsha::Eikev => "עקב",
                Parsha::Reeh => "ראה",
                Parsha::Shoftim => "שופטים",
                Parsha::KiSeitzei => "כי תצא",
                Parsha::KiSavoh => "כי תבוא",
                Parsha::NitzavimVayelech => "ניצבים/וילך",
                Parsha::Nitzavim => "ניצבים",
            },
            TorahReading::SpecialParsha(tr) => match tr {
                SpecialParsha::Zachor => "פרשת זכור",
                SpecialParsha::HaChodesh => "פרשת החודש",
                SpecialParsha::Parah => "פרשת פרה",
                SpecialParsha::Shekalim => "פרשת שקלים",
            },
        },
    }
}
