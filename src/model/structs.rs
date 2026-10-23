use slint::SharedString;

pub enum StateMessage {
    Idle,
    DownloadStart,
    DownloadParseUrl,
    DownloadParseLocalPath,
    DownloadFetchArchive,
    DownloadExtractArchive,
    Done,

    // ErrorFailedToParseUrl,
    // ErrorFailedToParseLocalPath,
    // ErrorFailedToFetchArcive,
    // ErrorFailedToExtractArchive,
    ErrorFailedToPickFolder,
}

impl StateMessage {
    pub fn to_shared_string(&self) -> SharedString {
        match self {
            Self::Idle => "Здесь будет отображаться состояние загрузки".into(),
            Self::DownloadStart => "Начало загрузки...".into(),
            Self::DownloadParseUrl => "Парсинг URL архива...".into(),
            Self::DownloadParseLocalPath => "...Парсинг целевого пути".into(),
            Self::DownloadFetchArchive => "Скачивание архива...".into(),
            Self::DownloadExtractArchive => "...Распаковка архива...".into(),
            Self::Done => "Готово! Шаблон доступен по указанному пути".into(),
            Self::ErrorFailedToPickFolder => format!("Не удалось открыть окно выбора директории").into(),
        }
    }
}
