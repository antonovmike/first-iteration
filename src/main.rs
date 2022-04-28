#![allow(unused)]

use geo::prelude::*;
use geo::point;
use carapax::types::{Message, MessageData};
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{ChatId, Text},
    Api, App, Context, ExecuteError, Ref,
};
use dotenv::dotenv;
use std::env;
use std::fmt;

struct CoffeeHouse {
    name: String,
    description: String,
    schedule: String,
    address: String,
    insta: String,
    location_x: f32,
    location_y: f32,
}

impl fmt::Display for CoffeeHouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}, {}", self.name, self.description, self.schedule, self.address, self.insta)
    }
}

fn kofe_list() -> [CoffeeHouse; 30] {
        let kmk: [CoffeeHouse; 30] = [CoffeeHouse {
            name: String::from("Stamba cafe"),
            description: String::from("
Stamba cafe
Двор с деревянным амфитеатром, в центре которого возвышается бутафорский электрический столб, лобби, напоминающее джунгли с неоновыми вывесками, книги, люстры — каждый метр хочется немножко умереть от эстетики. Здесь берем бенедикты, сырники c маскарпоне или йогурт с матчей + альтернативный кофе.
Режим работы c 8:00 до 2:00
Адрес: Merab Kostava St. 14
Инстаграм: https://www.instagram.com/cafe.stamba
Google maps: https://goo.gl/maps/hCTRnHS5NJYdLkNk9"),
            schedule: String::from("c 8:00 до 2:00"),
            address: String::from("Merab Kostava St. 14"),
            insta: String::from("https://www.instagram.com/cafe.stamba/"),
            location_x: 41.705732,
            location_y: 44.787975,
        },
        CoffeeHouse {
            name: String::from("Гардения Шеварнадзе"),
            description: String::from("
Гардения Шеварнадзе
Вообще, это сад-оранжерея. Выглядит сказочно: каменные домики, рояли в кустах, канарейки, клумбы из старой обуви и куча мелочей, которые порадуют вас и ваших подписчиков. Здесь же есть кафе со сладостями и бабушкинским компотом.
Режим работы c 10:00 до 18:00
Адрес: Khudadovi, 38
Инстаграм: https://www.instagram.com/gardeniashevardnadze
Google maps: https://goo.gl/maps/x3DWwAzxnvQ2CwmT6"),
            schedule: String::from("c 10:00 до 18:00"),
            address: String::from("Khudadovi, 38"),
            insta: String::from("https://www.instagram.com/gardeniashevardnadze/"),
            location_x: 41.731023,
            location_y: 44.830517,
        },
        CoffeeHouse {
            name: String::from("Фабрика"),
            description: String::from("
Фабрика
Это бывшее здание швейной фабрики: высокие потолки, простая и стильная отделка — все намекает на конструктивистское прошлое. Здесь хостел, бесплатная зона коворкинга и хенд-мейд магазинчики, есть даже барбершоп. Но это место больше, чем набор милых заведений, это точка притяжения тбилисской молодежи, экспатов и путешественников. За кофе и конфетками (!) идите в Milk Coffee.
Режим работы c 11-12, Milk будням открывается с 9
Адрес: Ninoshvili str.8
Инстаграм: http://instagram.com/fabrika_tbilisi
Google maps: "),
            schedule: String::from("c 11-12, Milk будням открывается с 9"),
            address: String::from("NINOSHVILI STR.8"),
            insta: String::from("http://instagram.com/fabrika_tbilisi/"),
            location_x: 41.709530,
            location_y: 44.802610,
        },
        CoffeeHouse {
            name: String::from("Erti Kava"),
            description: String::from("
Erti Kava
Одна из первых спешелти кофеен в Грузии. Сейчас это уже сеть кофеен (есть 2 точки в Тбилиси и 1 в Батуми). Сделана с большой любовью к Грузии: на стенах изображен гранат, а на стаканчиках — старый Тбилиси. Цены одни из самых высоких, но кофе и завтраки — достойные.
Режим работы c 8:00 до 21:00
Адрес: Mitropan Laghidze St, 8
Инстаграм: http://instagram.com/ertikava_coffeeroom
Google maps: "),
            schedule: String::from("c 8:00 до 21:00"),
            address: String::from("Mitropan Laghidze St, 8"),
            insta: String::from("http://instagram.com/ertikava_coffeeroom/?hl=en"),
            location_x: 41.698988,
            location_y: 44.795367,
        },
        CoffeeHouse {
            name: String::from("Stories"),
            description: String::from("
Stories
Кофейня в старом Тбилиси, где можно привычно литрами пить недорогой и вкусный фильтр-кофе. Здесь есть также вино и десерты, такое европейское местечко. Скидка 10% если вы приходите со своей кружкой.
Режим работы c 9:00 до 20:00 (в выходные с 10)
Адрес: 9 Galaktion Tabidze St
Инстаграм: https://www.instagram.com/stories.tbilisi/?hl=en
Google maps: "),
            schedule: String::from("c 9:00 до 20:00 (в выходные с 10)"),
            address: String::from("9 Galaktion Tabidze St"),
            insta: String::from("https://www.instagram.com/stories.tbilisi/?hl=en"),
            location_x: 41.691689,
            location_y: 44.801089,
        },
        CoffeeHouse {
            name: String::from("Coffee Lab"),
            description: String::from("
Coffee Lab
Двухэтажная кофейня в районе Сабуртало, что находится не совсем в центре, но популярен у экспатов. Это одна из немногих кофеен, которая не закупает зерна, а обжаривает сама, здесь же можно купить кофейник, керамическую воронку и другие аксессуары. Удобно, что на террасе есть розетки почти у каждого столика.
Режим работы c 9:00 до 22:00
Адрес: 27 Alexander Kazbegi Ave
Инстаграм: https://www.instagram.com/coffeelabgeorgia/?hl=en
Google maps: "),
            schedule: String::from("c 9:00 до 22:00"),
            address: String::from("27 Alexander Kazbegi Ave"),
            insta: String::from("https://www.instagram.com/coffeelabgeorgia/?hl=en"),
            location_x: 41.725648,
            location_y: 44.754978,
        },
        CoffeeHouse {
            name: String::from("Luicoffee"),
            description: String::from("
Luicoffee
Если вы любите авторские напитки и кофе с молоком, то вам сюда. Халва-латте, фисташковый раф, кофе с апельсиновым фрешем и тд. Здесь ОЧЕНЬ большое меню и можно купить даже борщ, что неудивительно — кафе открыла семья из Киева.
Режим работы круглосуточно
Адрес: 23-23a Chavchavadze Ave
Инстаграм: https://www.instagram.com/luicoffee
Google maps: https://goo.gl/maps/BEvtVVwNFtSoehjG7"),
            schedule: String::from("круглосуточно"),
            address: String::from("23-23a Chavchavadze Ave"),
            insta: String::from("https://www.instagram.com/luicoffee/"),
            location_x: 41.709292,
            location_y: 44.768015,
        },
        CoffeeHouse {
            name: String::from("Kikliko"),
            description: String::from("
Kikliko
Kikliko переводится как «кукареку». Это традиционное блюдо для завтрака, похоже на гренки с сыром. Здесь можно попробовать более 5 вариантов этого блюда. Есть уютная терраса в тени деревьев. Фильтр-кофе нет, есть только американо и производные эспрессо.
Режим работы с 8 до 15 (в выходные с 9 до 16)
Адрес: 28 Mtskheta St
Инстаграм: https://www.instagram.com/kikliko_
Google maps: "),
            schedule: String::from("с 8 до 15 (в выходные с 9 до 16)"),
            address: String::from("28 Mtskheta St"),
            insta: String::from("https://www.instagram.com/kikliko_/"),
            location_x: 41.705725,
            location_y: 44.769619,
        },
        CoffeeHouse {
            name: String::from("Lolita"),
            description: String::from("
Lolita
Современная Грузия, туристы и стиль в каждой детали. Вкусно, модно. громко, инстаграмно. Заказ можно делать через приложение без участия официанта. Есть недорогой фильтр-кофе, но очень маленького размера:(
Режим работы с 11:00 до 2:00
Адрес: 7 Tamar Chovelidze St
Инстаграм: https://www.instagram.com/lolita.tbilisi
Google maps: "),
            schedule: String::from("с 11:00 до 2:00"),
            address: String::from("7 Tamar Chovelidze St"),
            insta: String::from("https://www.instagram.com/lolita.tbilisi/"),
            location_x: 41.705671,
            location_y: 44.786925,
        },
        CoffeeHouse {
            name: String::from("Maria Magdalena"),
            description: String::from("
Maria Magdalena
Стильное место спрятанном в зеленом дворике. Европейская еда, боулы и все такое. В лавке при ресторане можно купить растение. Как найти? Поднимитесь от м.Руставели по улице Зандукели, оттуда первый поворот направо. Пройдите сквозь кирпичную арку и двигайтесь вниз по тропинке мимо «Кето и Коте», пока не обнаружите зелёную дверь.
Режим работы с 11:00 до 23:00
Адрес: 5 Mikheil Zandukeli Dead End
Инстаграм: https://www.instagram.com/mariamagdalina.tbilisi
Google maps: "),
            schedule: String::from("с 11:00 до 23:00"),
            address: String::from("5 Mikheil Zandukeli Dead End"),
            insta: String::from("https://www.instagram.com/mariamagdalina.tbilisi/"),
            location_x: 41.704922,
            location_y: 44.788100,
        },
        CoffeeHouse {
            name: String::from("Kikodze"),
            description: String::from("
Kikodze
Бар с завтраками и авторскими коктейлями в здании бывшего винного завода. Самое популярное и инстаграмное блюдо  — сырники. Пышные, воздушные, круглые со сметаной и вареньем.  Рекомендация для более голодных  —  Чижи-Бижи — грузинская версия шакшуки с очень большим количеством томатов.
Режим работы с 9:00 до 1:00
Адрес: 1 Vasil Petriashvili Street
Инстаграм: https://www.instagram.com/kikodzebar
Google maps: "),
            schedule: String::from("с 9:00 до 1:00"),
            address: String::from("1 Vasil Petriashvili Street"),
            insta: String::from("https://www.instagram.com/kikodzebar/"),
            location_x: 41.708215,
            location_y: 44.788091,
        },
        CoffeeHouse {
            name: String::from("Shavi Coffee"),
            description: String::from("
Shavi Coffee
Очень вкусный кофе, обжаривают зерна сами прямо при кофейне. Попробуйте ферментированные зерна Эфиопии! Приятный плейлист, домашняя выпечка  - идеальное место для начала дня. Здесь также можно купить аксессуары (например, аэропресс домой)
Режим работы с 8:00 до 21:00
Адрес: 40 Zandukeli Street
Инстаграм: https://www.instagram.com/shavi.coffee
Google maps: "),
            schedule: String::from("с 8:00 до 21:00"),
            address: String::from("40 Zandukeli Street"),
            insta: String::from("https://www.instagram.com/shavi.coffee/"),
            location_x: 41.704129,
            location_y: 44.784089,
        },
        CoffeeHouse {
            name: String::from("PULP"),
            description: String::from("
PULP
Кофейня с берлинским вайбом и очень вкусным тирамису. Кофе наливается в керамические кружки местного бренда 1300 Ceramic Studio.
Режим работы: Пн-Чт 9:30-19, Пт 9:30-20, Сб 10-20, Вс 10-18
Адрес: 22 Simon Janashia
Инстаграм: https://www.instagram.com/pulp.tbilisi/?hl=en
Google maps: "),
            schedule: String::from("Пн-Чт 9:30-19, Пт 9:30-20, Сб 10-20, Вс 10-18"),
            address: String::from("22 Simon Janashia"),
            insta: String::from("https://www.instagram.com/pulp.tbilisi/?hl=en"),
            location_x: 41.705987,
            location_y: 44.781482,
        },
        CoffeeHouse {
            name: String::from("They said books"),
            description: String::from("
They said books
Кофейня при книжном с лучшим в Тбилиси морковным тортом. Столиков немного, можно в тишине полистать книгу и выпить кофе на зернах из Coffee LAB (местный обжарщик с кофейней в Сабуртало)
Режим работы с 12 до 21
Адрес: 10 Giorgi Akhvlediani St
Инстаграм: https://www.instagram.com/theysaidbooks.coffeeshop
Google maps: "),
            schedule: String::from("с 12 до 21"),
            address: String::from("10 Giorgi Akhvlediani St"),
            insta: String::from("https://www.instagram.com/theysaidbooks.coffeeshop/"),
            location_x: 41.705315,
            location_y: 44.789875,
        },
        CoffeeHouse {
            name: String::from("Daily grind"),
            description: String::from("
Daily grind
Лучший кофе недалеко от площади Свободы. Зерна - шведский бренд Koppi и местные Shavi Coffee Roasters, готовят их с помощью красивой машины La Marzocco. Шоколадный торт и яблочный пирог - топ. Панорамные окна с видом на старый город и  красивая плитка на полу - тоже топ!
Режим работы с 8 до 12
Адрес: 4 Kote Afkhazi St
Инстаграм: https://www.instagram.com/dailygrindtbilisi
Google maps: https://goo.gl/maps/X4DbGjH63gpp91m19"),
            schedule: String::from("с 8 до 12"),
            address: String::from("4 Kote Afkhazi St"),
            insta: String::from("https://www.instagram.com/dailygrindtbilisi/"),
            location_x: 41.693138,
            location_y: 44.802658,
        },
        CoffeeHouse {
            name: String::from("Black Cup x Valiko Bar"),
            description: String::from("
Black Cup x Valiko Bar

Режим работы с 18 до 23:00
Адрес: 24 Galaktion Tabidze Street
Инстаграм: https://www.instagram.com/valiko.mansion
Google maps: "),
            schedule: String::from("с 18 до 23:00"),
            address: String::from("24 Galaktion Tabidze Street"),
            insta: String::from("https://www.instagram.com/valiko.mansion/"),
            location_x: 41.689804,
            location_y: 44.801400,
        },
        CoffeeHouse {
            name: String::from("Eleven.window"),
            description: String::from("
Eleven.window
Это место находится всего в двух шагах от верхней станции фуникулера. Отличная кофейня 3-й волны, где кофе и вкусности готовятся с любовью. Плюс прекрасный магазин и место для ивентов. Гостеприимные хосты проводят ярмарки и поддерживают местных художников.
Режим работы с 10 до 21:00
Адрес: 6 Niaghvari St
Инстаграм: https://www.instagram.com/eleven.window
Google maps: https://goo.gl/maps/pxGFnwdBR1SbbKmX6"),
            schedule: String::from("с 10 до 21:00"),
            address: String::from("6 Niaghvari St"),
            insta: String::from("https://www.instagram.com/eleven.window/"),
            location_x: 41.695242,
            location_y: 44.793238,
        },
        CoffeeHouse {
            name: String::from("Nur"),
            description: String::from("
Nur
Спешалти микро-кофейня с приятными ценами. Можно купить зерна Neighbourhood Coffee Roasters и съесть вкусное овсяное печенье за 3 лари. Гости хвалят какао и чизкейк.
Режим работы в будни с 10:00 - 19:00 (выходные 11:00 - 18:00)
Адрес: Lado Kavsadze 7
Инстаграм: https://www.instagram.com/nur_coffeeshop
Google maps: "),
            schedule: String::from("в будни с 10:00 - 19:00 (выходные 11:00 - 18:00)"),
            address: String::from("Lado Kavsadze 7"),
            insta: String::from("https://www.instagram.com/nur_coffeeshop/"),
            location_x: 41.708684,
            location_y: 44.761402,
        },
        CoffeeHouse {
            name: String::from("Lamarzocco"),
            description: String::from("
Lamarzocco
Небольшой кофе-бар в Ваке, кофейня от ребят, которые открыли Daily grind. Спешалти кофе, вкусный флэтуайт, несколько видов домашних веганских десертов (говорят, самое вкусное – морковный кекс). Можно попробовать вкусную местную лимончеллу. Кофейня на 4-6 посадочных мест, но перед входом есть мини-сквер со скамейками и парой пляжных кресел.
Режим работы с 8:30 до 18:30 (по выходным работают до 21)
Адрес: Ilia Chavchavadze 27
Инстаграм: https://www.instagram.com/lamarzoccoespressobar
Google maps: https://goo.gl/maps/oRNzGXPBcths3eDL6"),
            schedule: String::from("с 8:30 до 18:30 (по выходным работают до 21)"),
            address: String::from("Ilia Chavchavadze 27"),
            insta: String::from("https://www.instagram.com/lamarzoccoespressobar/"),
            location_x: 41.710275,
            location_y: 44.766336,
        },
        CoffeeHouse {
            name: String::from("Jupiter"),
            description: String::from("
Jupiter
Сыры, хлеб, хумус, V-60, матча и локал варенье. Владелец очень гостеприимный. Цены на кофе выше среднего. Но все же сюда стоит заглянуть. Конечно, в наличии вино, мы в Грузии или где🕺
Режим работы с 8:00 до 23:00
Адрес: Ivane Machabeli 4
Инстаграм: https://www.instagram.com/jupiter.tbilisi
Google maps: "),
            schedule: String::from("с 8:00 до 23:00"),
            address: String::from("Ivane Machabeli 4"),
            insta: String::from("https://www.instagram.com/jupiter.tbilisi/"),
            location_x: 41.692065,
            location_y: 44.799947,
        },
        CoffeeHouse {
            name: String::from("Sol • სოლ"),
            description: String::from("
Sol • სოლ
Завтраки и коктейли целый день. Стильное, яркое место для завтрака. Есть приятный дворик. Брать – смузи, завтраки, капучино и пить коктейли.
Режим работы с 8:00 до 22:00
Адрес: 28 Vasil Petriashvili Street
Инстаграм: https://www.instagram.com/sol.tbilisi/?hl=ru
Google maps: "),
            schedule: String::from("с 8:00 до 22:00"),
            address: String::from("28 Vasil Petriashvili Street, Tbilisi 0179"),
            insta: String::from("https://www.instagram.com/sol.tbilisi/?hl=ru"),
            location_x: 41.705638,
            location_y: 44.779638,
        },
        CoffeeHouse {
            name: String::from("Mondo Coffee"),
            description: String::from("
Mondo Coffee
Основатель компании Александр Беридзе пристрастился эспрессо, живя в Италии. И решил создать первый грузинский бренд, который бы импортировал итальянский кофе премиум-класса. В 2015 году была основана компания Mondo и сейчас бренд представлен в США и Грузии. Здесь огромный выбор дрип-кофе!
Режим работы с 9:00 до 20:00
Адрес: 13 a Ivane Tarkhnishvili St
Инстаграм: https://www.instagram.com/mondogeorgia
Google maps: "),
            schedule: String::from("с 9:00 до 20:00"),
            address: String::from("13 a Ivane Tarkhnishvili St, Tbilisi"),
            insta: String::from("https://www.instagram.com/mondogeorgia/"),
            location_x: 41.706474,
            location_y: 44.783221,
        },
        CoffeeHouse {
            name: String::from("Shukura"),
            description: String::from("
Shukura
Спешалти кофейня со вкусными завтраками и небольшой барной картой. Летом особенно приятно, можно расположиться снаружи вокруг зелени и гирлянд (места мало, вместо столиков скамейки, но очень уютно).
Режим работы с 8:00 до 23:00
Адрес: 49 Irakli Abashidze Street
Инстаграм: https://www.instagram.com/shukura.coffee/?hl=ru
Google maps: "),
            schedule: String::from("с 8:00 до 23:00"),
            address: String::from("49 Irakli Abashidze Street, Tbilisi 0162"),
            insta: String::from("https://www.instagram.com/shukura.coffee/?hl=ru"),
            location_x: 41.708367,
            location_y: 44.760192,
        },
        CoffeeHouse {
            name: String::from("Erti Kava"),
            description: String::from("
Erti Kava
Одна из первых спешелти кофеен в Грузии. Сейчас это уже сеть кофеен (есть 2 точки в Тбилиси и 1 в Батуми). Сделана с большой любовью к Грузии: на стенах изображен гранат, а на стаканчиках — старый Тбилиси. Цены одни из самых высоких, но кофе и завтраки — достойные.
Режим работы с 8:00 до 21:00
Адрес: 81 Irakli Abashidze Street
Инстаграм: http://instagram.com/ertikava_coffeeroom/?hl=en
Google maps: "),
            schedule: String::from("с 8:00 до 21:00"),
            address: String::from("81 Irakli Abashidze Street, Tbilisi 0162"),
            insta: String::from("http://instagram.com/ertikava_coffeeroom/?hl=en"),
            location_x: 41.708954,
            location_y: 44.755021,
        },
        CoffeeHouse {
            name: String::from("Books from past"),
            description: String::from("
Books from past
Кофейня и книжный магазин в тбилисской квартире.Аристократическое сочетание книг, окон в полный рост, скрипящего паркета, переходящего в бело-голубую плитку и белоснежной необъятной тюли😍 В меню комбуча и инстаграмные десерты. Ну и кофе здесь отличный!
Режим работы с 12:00 до 20:00
Адрес: 10 Giorgi Akhvlediani St, flat 2
Инстаграм: https://www.instagram.com/booksfrompast.shop
Google maps: "),
            schedule: String::from("с 12:00 до 20:00"),
            address: String::from("10 Giorgi Akhvlediani St, flat 2."),
            insta: String::from("https://www.instagram.com/booksfrompast.shop/"),
            location_x: 41.705389,
            location_y: 44.789874,
        },
        CoffeeHouse {
            name: String::from(" "),
            description: String::from("
Depo
Неприметное здание с приятным пространством внутри и летней веранда на крыше (тбилисский руфтоп, не имеющий ничего общего с московским фудкортом). Регулярно что-то происходит — диджей-сеты, маркеты, благотворительные ивенты. С 10:00 до 14:00 сервируют завтраки, а обеды —  с 12:00 до 16:00 по будням (2 блюда за 20Gel, топовый томатный копчёный суп).
Режим работы с 10 до полуночи
Адрес: Shalva Ghambashldze, 10
Инстаграм: https://www.instagram.com/depo.tbilisi/
Google maps: https://goo.gl/maps/Wj4kqwcTjEJ9KVm36"),
            schedule: String::from(" "),
            address: String::from(" "),
            insta: String::from(" "),
            location_x: 41.708348,
            location_y: 44.784006,
        },
        CoffeeHouse {
            name: String::from(" "),
            description: String::from("
Unity kava
ComUNITY of specialty coffee admirers — указано в инстаграме проекта. Кофейня открылась 25 апреля 2022г. Пэт и альтернатива фрэндли. Возьмите надуги чизкейк и садитесь на крошечную солнечную террасу наблюдать за песиками.
Режим работы с 8 до 15:00 (т.к. кофейня только появилась, режим может меняться, проверяйте в инстаграм)
Адрес: Shalva Dadiani, 20
Инстаграм: https://www.instagram.com/unity_kava/
Google maps: https://goo.gl/maps/uLTnN8h7vvWC6fkLA"),
            schedule: String::from(" "),
            address: String::from(" "),
            insta: String::from(" "),
            location_x: 41.691128,
            location_y: 44.802190,
        },
        CoffeeHouse {
            name: String::from(" "),
            description: String::from("
APC
Новая (открылась в апреле 2022) французская булочная в районе Ваке.  По совместительству картинная галерея. Владелица  — профессиональный кондитер из Франции Jullian Hequet. Эклеры, тарталетки, улитки и, конечно, круассаны. Кофейная карта как в Париже: крепкий эспрессо и латте в высоких бокалах.
Режим работы: Вт-Чт 10:00-20:00, Пт-Вс 11:00-21:00
Адрес: 10 I.Abashidze 
Инстаграм: https://www.instagram.com/apc.georgia/
Google maps: https://goo.gl/maps/fpUgRZEc15Akw7GN7"),
            schedule: String::from(" "),
            address: String::from(" "),
            insta: String::from(" "),
            location_x: 41.706840,
            location_y: 44.772267,
        },
        CoffeeHouse {
            name: String::from(" "),
            description: String::from("
Sablée
Булочная в самом начале Ваке. Всевозможные десерты, баночки с вареньем и соленой карамелью, конфеты, печенье, хлеб (на пасху ребята делали куличи и творожную пасху), можно заказать целый торт. Не знаете с чего начать — берите слойку со сливочно-лимонным кремом и миндалем. Есть сытная еда (сэндвичи и салаты). Espresso-based кофейное меню.
Режим работы c 09:00 до 22:00
Адрес: Kekelidze street 1
Фейсбук: https://www.facebook.com/sableebakery/
Google maps: https://goo.gl/maps/SqyQqSWqJLPBhq298"),
            schedule: String::from(" "),
            address: String::from(" "),
            insta: String::from(" "),
            location_x: 41.708389,
            location_y: 44.778489,
        },
        CoffeeHouse {
            name: String::from(" "),
            description: String::from("
Groovy roasters
Спешелти-кофейня недалеко от людной Fabrika. Ходит молва про местный шоколадный веганский кекс с вишней и свёклой. Если не хотите сладкое, то берите сэндвич с карамелизированным сыром и хлебом на закваске. Большой выбор кофейного зерна (которые они сами обжаривают). Есть эспрессо-тоник, вино, пиво и даже самбука. Место новое, так что ждем расширения ассортимента меню.
Режим работы с 8 до 8
Адрес: Ninoshvili 17
Инстаграм: https://www.instagram.com/groovyroasters/
Google maps: https://goo.gl/maps/2CVxQkwiHB1sbnhb6"),
            schedule: String::from(" "),
            address: String::from(" "),
            insta: String::from(" "),
            location_x: 41.711521,
            location_y: 44.801714,
        },
    ];
    kmk
}

fn distance(lat_user: f32, lon_user: f32) -> String {
    dbg!(&lat_user);
    dbg!(&lon_user);
    let mut temporary_collection = vec![];

    let point_user = point!(x: lat_user, y: lon_user);
// ITERATION
    for index in 0..kofe_list().len() {
        let point_destination = point!(x: kofe_list()[index].location_x, y: kofe_list()[index].location_y);
        let calculated_distance: i32 = point_user.haversine_distance(&point_destination).round() as i32;
        temporary_collection.push((calculated_distance, kofe_list()[index].description.clone()));
    }
    temporary_collection.sort_by(|a, b| a.0.cmp(&b.0));
    let three = format!("{}, {}, {}", temporary_collection[0].1, temporary_collection[1].1, temporary_collection[2].1);
    // temporary_collection[0].1.to_string()
    three
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
    dbg!(&message);
    if let MessageData::Location(location) = message.data {
        let lon = location.longitude;
        let lat = location.latitude;
        let calculated_distance = distance(lon, lat);
        let method = SendMessage::new(chat_id, calculated_distance);
        api.execute(method).await?;
    };
    dbg!("F");
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await
}
