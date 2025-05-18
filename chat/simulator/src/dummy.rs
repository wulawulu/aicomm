use fake::Dummy;
use rand::seq::IndexedRandom as _;

pub struct AppVersion;
pub struct SystemOs;
pub struct SystemArch;
pub struct SystemLocale;
pub struct SystemTimezone;
pub struct RegionName;
pub struct MessageType;

impl Dummy<AppVersion> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &AppVersion, rng: &mut R) -> Self {
        let major = rng.random_range(1..=14);
        let minor = rng.random_range(0..=99);
        let patch = rng.random_range(0..=99);
        format!("{}.{}.{}", major, minor, patch)
    }
}

impl Dummy<SystemOs> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemOs, rng: &mut R) -> Self {
        let os = ["macOS", "Linux", "Windows", "iOS", "Android"]
            .choose(rng)
            .unwrap();
        os.to_string()
    }
}

impl Dummy<SystemArch> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemArch, rng: &mut R) -> Self {
        let arch = ["x86_64", "aarch64"].choose(rng).unwrap();
        arch.to_string()
    }
}

impl Dummy<SystemLocale> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemLocale, rng: &mut R) -> Self {
        let locale = [
            "en_US", "en_GB", "fr_FR", "ru_RU", "zh_CN", "ja_JP", "ko_KR", "zh_TW", "zh_HK",
        ]
        .choose(rng)
        .unwrap();
        locale.to_string()
    }
}
impl Dummy<SystemTimezone> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &SystemTimezone, rng: &mut R) -> Self {
        let timezone = [
            "America/New_York",
            "America/Los_Angeles",
            "America/Chicago",
            "America/Denver",
            "America/Phoenix",
            "America/Anchorage",
            "America/Adak",
            "America/New_York",
            "Europe/London",
            "Europe/Paris",
            "Europe/Berlin",
            "Europe/Madrid",
            "Asia/Shanghai",
            "Asia/Tokyo",
            "Asia/Seoul",
            "Asia/Hong_Kong",
            "Asia/Singapore",
            "Asia/Dubai",
            "Asia/Istanbul",
            "Asia/Kolkata",
            "Asia/Kuala_Lumpur",
            "Asia/Taipei",
            "Asia/Seoul",
            "Asia/Shanghai",
            "Asia/Tokyo",
            "Asia/Hong_Kong",
            "Asia/Singapore",
            "Asia/Dubai",
            "Asia/Istanbul",
            "Asia/Kolkata",
            "Asia/Kuala_Lumpur",
            "Asia/Taipei",
        ]
        .choose(rng)
        .unwrap();
        timezone.to_string()
    }
}

impl Dummy<RegionName> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &RegionName, rng: &mut R) -> Self {
        let region = ["California", "New York", "Texas", "Florida"]
            .choose(rng)
            .unwrap();
        region.to_string()
    }
}

impl Dummy<MessageType> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &MessageType, rng: &mut R) -> Self {
        let message_type = ["text", "image", "audio", "video"].choose(rng).unwrap();
        message_type.to_string()
    }
}
