use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

use crate::room::Room;

#[derive(EnumIter, EnumCount, Hash, Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Spatula {
    // Bikini Bottom
    OnTopOfThePineapple,
    OnTopOfShadyShoals,
    OnTopOfTheChumBucket,
    SpongebobsCloset,
    AnnoySquidward,
    AmbushAtTheTreeDome,
    InfestationAtTheKrustyKrab,
    AWallJumpInTheBucket,

    // Jellyfish Fields
    TopOfTheHill,
    CowaBungee,
    Spelunking,
    PatricksDilemma,
    NavigateTheCanyonsAndMesas,
    DrainTheLake,
    SlideLeap,
    DefeatKingJellyfish,

    // Downtown Bikini Bottom
    EndOfTheRoad,
    LearnSandysMoves,
    TikisGoBoom,
    AcrossTheRooftops,
    SwinginSandy,
    AmbushInTheLighthouse,
    ExtremeBungee,
    ComeBackWithTheCruiseBubble,

    // Goo Lagoon
    KingOfTheCastle,
    ConnectTheTowers,
    SaveTheChildren,
    OverTheMoat,
    ThroughTheSeaCaves,
    CleanOutTheBumperBoats,
    SlipAndSlideUnderThePier,
    TowerBungee,

    // Poseidome
    RumbleAtThePoseidome,

    // Rock Bottom
    GetToTheMuseum,
    SlipSlidingAway,
    ReturnTheMuseumsArt,
    SwingalongSpatula,
    PlunderingRobotsInTheMuseum,
    AcrossTheTrenchOfDarkness,
    LasersAreFunAndGoodForYou,
    HowInTarnationDoYouGetThere,

    // Mermalair
    TopOfTheEntranceAreaML,
    TopOfTheComputerArea,
    ShutDownTheSecuritySystem,
    TheFunnelMachines,
    TheSpinningTowersOfPower,
    TopOfTheSecurityTunnel,
    CompleteTheRollingBallRoom,
    DefeatPrawn,

    // Sand Mountain
    FrostyBungee,
    TopOfTheLodge,
    DefeatRobotsOnGuppyMound,
    BeatMrsPuffsTime,
    DefeatRobotsOnFlounderHill,
    BeatBubbleBuddysTime,
    DefeatRobotsOnSandMountain,
    BeatLarrysTime,

    // Industrial Park
    RoboPatrickAhoy,

    // Kelp Forest
    ThroughTheWoods,
    FindAllTheLostCampers,
    TikiRoundup,
    DownInTheSwamp,
    ThroughTheKelpCaves,
    PowerCrystalCrisis,
    KelpVineSlide,
    BeatMermaidMansTime,

    // Flying Dutchman's Graveyard
    TopOfTheEntranceAreaFDG,
    APathThroughTheGoo,
    GooTankerAhoy,
    TopOfTheStackOfShips,
    ShipwreckBungee,
    DestroyTheRobotShip,
    GetAloftThereMatey,
    DefeatTheFlyingDutchman,

    // SpongeBob's Dream
    AcrossTheDreamscape,
    FollowTheBouncingBall,
    SlidingTexasStyle,
    SwingersAhoy,
    MusicIsInTheEarOfTheBeholder,
    KrabbyPattyPlatforms,
    SuperBounce,
    HereYouGo,

    // Chum Bucket Lab
    KahRahTae,
    TheSmallShallRuleOrNot,
}

impl Spatula {
    pub fn get_offset(&self) -> Option<usize> {
        match *self {
            Spatula::OnTopOfThePineapple => Some(0xa8),
            Spatula::OnTopOfShadyShoals => Some(0xcf),
            Spatula::OnTopOfTheChumBucket => Some(0xd0),
            Spatula::SpongebobsCloset => Some(0x5d),
            Spatula::AnnoySquidward => Some(0x26),
            Spatula::AmbushAtTheTreeDome => Some(0x3a),
            Spatula::InfestationAtTheKrustyKrab => Some(0xce),
            Spatula::AWallJumpInTheBucket => Some(0x2a),
            Spatula::TopOfTheHill => Some(0xc8),
            Spatula::CowaBungee => Some(0xc9),
            Spatula::Spelunking => Some(0xd8),
            Spatula::PatricksDilemma => Some(0xd7),
            Spatula::NavigateTheCanyonsAndMesas => Some(0xfa),
            Spatula::DrainTheLake => Some(0xea),
            Spatula::SlideLeap => Some(0x58),
            Spatula::DefeatKingJellyfish => Some(0x128),
            Spatula::EndOfTheRoad => Some(0xba),
            Spatula::LearnSandysMoves => Some(0xb9),
            Spatula::TikisGoBoom => Some(0x111),
            Spatula::AcrossTheRooftops => Some(0xab),
            Spatula::SwinginSandy => Some(0xac),
            Spatula::AmbushInTheLighthouse => Some(0x53),
            Spatula::ExtremeBungee => Some(0x99),
            Spatula::ComeBackWithTheCruiseBubble => Some(0x9a),
            Spatula::KingOfTheCastle => Some(0x12a),
            Spatula::ConnectTheTowers => Some(0x154),
            Spatula::SaveTheChildren => Some(0x153),
            Spatula::OverTheMoat => Some(0x12b),
            Spatula::ThroughTheSeaCaves => Some(0x5c),
            Spatula::CleanOutTheBumperBoats => Some(0xff),
            Spatula::SlipAndSlideUnderThePier => Some(0xfd),
            Spatula::TowerBungee => Some(0xfe),
            Spatula::RumbleAtThePoseidome => Some(0x28),
            Spatula::GetToTheMuseum => Some(0xff),
            Spatula::SlipSlidingAway => Some(0xfe),
            Spatula::ReturnTheMuseumsArt => Some(0x105),
            Spatula::SwingalongSpatula => Some(0x107),
            Spatula::PlunderingRobotsInTheMuseum => Some(0x76),
            Spatula::AcrossTheTrenchOfDarkness => Some(0xa5),
            Spatula::LasersAreFunAndGoodForYou => Some(0xa4),
            Spatula::HowInTarnationDoYouGetThere => Some(0xa3),
            Spatula::TopOfTheEntranceAreaML => Some(0x72),
            Spatula::TopOfTheComputerArea => Some(0x6a),
            Spatula::ShutDownTheSecuritySystem => Some(0x6b),
            Spatula::TheFunnelMachines => Some(0x68),
            Spatula::TheSpinningTowersOfPower => Some(0x69),
            Spatula::TopOfTheSecurityTunnel => Some(0x9a),
            Spatula::CompleteTheRollingBallRoom => Some(0x45),
            Spatula::DefeatPrawn => Some(0x39),
            Spatula::FrostyBungee => Some(0x5d),
            Spatula::TopOfTheLodge => Some(0x5e),
            Spatula::DefeatRobotsOnGuppyMound => Some(0x91),
            Spatula::BeatMrsPuffsTime => Some(0x92),
            Spatula::DefeatRobotsOnFlounderHill => Some(0xa8),
            Spatula::BeatBubbleBuddysTime => Some(0xa9),
            Spatula::DefeatRobotsOnSandMountain => Some(0xcd),
            Spatula::BeatLarrysTime => Some(0xcc),
            Spatula::RoboPatrickAhoy => Some(0x28),
            Spatula::ThroughTheWoods => Some(0x94),
            Spatula::FindAllTheLostCampers => Some(0x8d),
            Spatula::TikiRoundup => Some(0x83),
            Spatula::DownInTheSwamp => Some(0x84),
            Spatula::ThroughTheKelpCaves => Some(0x5a),
            Spatula::PowerCrystalCrisis => Some(0x53),
            Spatula::KelpVineSlide => Some(0x53),
            Spatula::BeatMermaidMansTime => Some(0x54),
            Spatula::TopOfTheEntranceAreaFDG => Some(0x70),
            Spatula::APathThroughTheGoo => Some(0x71),
            Spatula::GooTankerAhoy => Some(0x6f),
            Spatula::TopOfTheStackOfShips => Some(0x86),
            Spatula::ShipwreckBungee => Some(0x87),
            Spatula::DestroyTheRobotShip => Some(0x5f),
            Spatula::GetAloftThereMatey => Some(0x60),
            Spatula::DefeatTheFlyingDutchman => Some(0x35),
            Spatula::AcrossTheDreamscape => Some(0x5e),
            Spatula::FollowTheBouncingBall => Some(0x5f),
            Spatula::SlidingTexasStyle => Some(0xa1),
            Spatula::SwingersAhoy => Some(0xa3),
            Spatula::MusicIsInTheEarOfTheBeholder => Some(0x22e),
            Spatula::KrabbyPattyPlatforms => Some(0x7f),
            Spatula::SuperBounce => Some(0x6e),
            Spatula::HereYouGo => Some(0x32),
            Spatula::KahRahTae => None,
            Spatula::TheSmallShallRuleOrNot => None,
        }
    }

    /// Returns the room this spatula is in.
    pub fn get_room(&self) -> Room {
        match *self {
            Spatula::OnTopOfThePineapple => Room::BikiniBottom,
            Spatula::OnTopOfShadyShoals => Room::BikiniBottom,
            Spatula::OnTopOfTheChumBucket => Room::BikiniBottom,
            Spatula::SpongebobsCloset => Room::SpongebobHouse,
            Spatula::AnnoySquidward => Room::SquidwardHouse,
            Spatula::AmbushAtTheTreeDome => Room::Treedome,
            Spatula::InfestationAtTheKrustyKrab => Room::BikiniBottom,
            Spatula::AWallJumpInTheBucket => Room::ChumBucket,
            Spatula::TopOfTheHill => Room::JellyfishRock,
            Spatula::CowaBungee => Room::JellyfishRock,
            Spatula::Spelunking => Room::JellyfishCaves,
            Spatula::PatricksDilemma => Room::JellyfishCaves,
            Spatula::NavigateTheCanyonsAndMesas => Room::JellyfishLake,
            Spatula::DrainTheLake => Room::JellyfishLake,
            Spatula::SlideLeap => Room::JellyfishMountain,
            Spatula::DefeatKingJellyfish => Room::JellyfishRock,
            Spatula::EndOfTheRoad => Room::DowntownStreets,
            Spatula::LearnSandysMoves => Room::DowntownStreets,
            Spatula::TikisGoBoom => Room::DowntownStreets,
            Spatula::AcrossTheRooftops => Room::DowntownRooftops,
            Spatula::SwinginSandy => Room::DowntownRooftops,
            Spatula::AmbushInTheLighthouse => Room::DowntownLighthouse,
            Spatula::ExtremeBungee => Room::DowntownSeaNeedle,
            Spatula::ComeBackWithTheCruiseBubble => Room::DowntownSeaNeedle,
            Spatula::KingOfTheCastle => Room::GooLagoonBeach,
            Spatula::ConnectTheTowers => Room::GooLagoonBeach,
            Spatula::SaveTheChildren => Room::GooLagoonBeach,
            Spatula::OverTheMoat => Room::GooLagoonBeach,
            Spatula::ThroughTheSeaCaves => Room::GooLagoonCaves,
            Spatula::CleanOutTheBumperBoats => Room::GooLagoonPier,
            Spatula::SlipAndSlideUnderThePier => Room::GooLagoonPier,
            Spatula::TowerBungee => Room::GooLagoonPier,
            Spatula::RumbleAtThePoseidome => Room::Poseidome,
            Spatula::GetToTheMuseum => Room::RockBottomDowntown,
            Spatula::SlipSlidingAway => Room::RockBottomDowntown,
            Spatula::ReturnTheMuseumsArt => Room::RockBottomDowntown,
            Spatula::SwingalongSpatula => Room::RockBottomDowntown,
            Spatula::PlunderingRobotsInTheMuseum => Room::RockBottomMuseum,
            Spatula::AcrossTheTrenchOfDarkness => Room::RockBottomTrench,
            Spatula::LasersAreFunAndGoodForYou => Room::RockBottomTrench,
            Spatula::HowInTarnationDoYouGetThere => Room::RockBottomTrench,
            Spatula::TopOfTheEntranceAreaML => Room::MermalairEntranceArea,
            Spatula::TopOfTheComputerArea => Room::MermalairMainChamber,
            Spatula::ShutDownTheSecuritySystem => Room::MermalairMainChamber,
            Spatula::TheFunnelMachines => Room::MermalairMainChamber,
            Spatula::TheSpinningTowersOfPower => Room::MermalairMainChamber,
            Spatula::TopOfTheSecurityTunnel => Room::MermalairSecurityTunnel,
            Spatula::CompleteTheRollingBallRoom => Room::MermalairBallroom,
            Spatula::DefeatPrawn => Room::MermalairVillianContainment,
            Spatula::FrostyBungee => Room::SandMountainHub,
            Spatula::TopOfTheLodge => Room::SandMountainHub,
            Spatula::DefeatRobotsOnGuppyMound => Room::SandMountainSlide1,
            Spatula::BeatMrsPuffsTime => Room::SandMountainSlide1,
            Spatula::DefeatRobotsOnFlounderHill => Room::SandMountainSlide2,
            Spatula::BeatBubbleBuddysTime => Room::SandMountainSlide2,
            Spatula::DefeatRobotsOnSandMountain => Room::SandMountainSlide3,
            Spatula::BeatLarrysTime => Room::SandMountainSlide3,
            Spatula::RoboPatrickAhoy => Room::IndustrialPark,
            Spatula::ThroughTheWoods => Room::KelpForest,
            Spatula::FindAllTheLostCampers => Room::KelpForest,
            Spatula::TikiRoundup => Room::KelpSwamps,
            Spatula::DownInTheSwamp => Room::KelpSwamps,
            Spatula::ThroughTheKelpCaves => Room::KelpCaves,
            Spatula::PowerCrystalCrisis => Room::KelpCaves,
            Spatula::KelpVineSlide => Room::KelpVines,
            Spatula::BeatMermaidMansTime => Room::KelpVines,
            Spatula::TopOfTheEntranceAreaFDG => Room::GraveyardLake,
            Spatula::APathThroughTheGoo => Room::GraveyardLake,
            Spatula::GooTankerAhoy => Room::GraveyardLake,
            Spatula::TopOfTheStackOfShips => Room::GraveyardShipwreck,
            Spatula::ShipwreckBungee => Room::GraveyardShipwreck,
            Spatula::DestroyTheRobotShip => Room::GraveyardShip,
            Spatula::GetAloftThereMatey => Room::GraveyardShip,
            Spatula::DefeatTheFlyingDutchman => Room::GraveyardBoss,
            Spatula::AcrossTheDreamscape => Room::SpongebobsDream,
            Spatula::FollowTheBouncingBall => Room::SpongebobsDream,
            Spatula::SlidingTexasStyle => Room::SandysDream,
            Spatula::SwingersAhoy => Room::SandysDream,
            Spatula::MusicIsInTheEarOfTheBeholder => Room::SquidwardsDream,
            Spatula::KrabbyPattyPlatforms => Room::KrabsDream,
            Spatula::SuperBounce => Room::SpongebobsDream,
            Spatula::HereYouGo => Room::PatricksDream,
            Spatula::KahRahTae => Room::ChumBucketLab,
            Spatula::TheSmallShallRuleOrNot => Room::ChumBucketBrain,
        }
    }
}

impl TryFrom<(u32, u32)> for Spatula {
    type Error = &'static str;

    fn try_from(value: (u32, u32)) -> Result<Self, Self::Error> {
        use Spatula::*;
        match value {
            (0, 0) => Ok(OnTopOfThePineapple),
            (0, 1) => Ok(OnTopOfShadyShoals),
            (0, 2) => Ok(OnTopOfTheChumBucket),
            (0, 3) => Ok(SpongebobsCloset),
            (0, 4) => Ok(AnnoySquidward),
            (0, 5) => Ok(AmbushAtTheTreeDome),
            (0, 6) => Ok(InfestationAtTheKrustyKrab),
            (0, 7) => Ok(AWallJumpInTheBucket),

            // Jellyfish Fields
            (1, 0) => Ok(TopOfTheHill),
            (1, 1) => Ok(CowaBungee),
            (1, 2) => Ok(Spelunking),
            (1, 3) => Ok(PatricksDilemma),
            (1, 4) => Ok(NavigateTheCanyonsAndMesas),
            (1, 5) => Ok(DrainTheLake),
            (1, 6) => Ok(SlideLeap),
            (1, 7) => Ok(DefeatKingJellyfish),

            // Downtown Bikini Bottom
            (2, 0) => Ok(EndOfTheRoad),
            (2, 1) => Ok(LearnSandysMoves),
            (2, 2) => Ok(TikisGoBoom),
            (2, 3) => Ok(AcrossTheRooftops),
            (2, 4) => Ok(SwinginSandy),
            (2, 5) => Ok(AmbushInTheLighthouse),
            (2, 6) => Ok(ExtremeBungee),
            (2, 7) => Ok(ComeBackWithTheCruiseBubble),

            // Goo Lagoon
            (3, 0) => Ok(KingOfTheCastle),
            (3, 1) => Ok(ConnectTheTowers),
            (3, 2) => Ok(SaveTheChildren),
            (3, 3) => Ok(OverTheMoat),
            (3, 4) => Ok(ThroughTheSeaCaves),
            (3, 5) => Ok(CleanOutTheBumperBoats),
            (3, 6) => Ok(SlipAndSlideUnderThePier),
            (3, 7) => Ok(TowerBungee),

            // Poseidome
            (4, 0) => Ok(RumbleAtThePoseidome),

            // Rock Bottom
            (5, 0) => Ok(GetToTheMuseum),
            (5, 1) => Ok(SlipSlidingAway),
            (5, 2) => Ok(ReturnTheMuseumsArt),
            (5, 3) => Ok(SwingalongSpatula),
            (5, 4) => Ok(PlunderingRobotsInTheMuseum),
            (5, 5) => Ok(AcrossTheTrenchOfDarkness),
            (5, 6) => Ok(LasersAreFunAndGoodForYou),
            (5, 7) => Ok(HowInTarnationDoYouGetThere),

            // Mermalair
            (6, 0) => Ok(TopOfTheEntranceAreaML),
            (6, 1) => Ok(TopOfTheComputerArea),
            (6, 2) => Ok(ShutDownTheSecuritySystem),
            (6, 3) => Ok(TheFunnelMachines),
            (6, 4) => Ok(TheSpinningTowersOfPower),
            (6, 5) => Ok(TopOfTheSecurityTunnel),
            (6, 6) => Ok(CompleteTheRollingBallRoom),
            (6, 7) => Ok(DefeatPrawn),

            // Sand Mountain
            (7, 0) => Ok(FrostyBungee),
            (7, 1) => Ok(TopOfTheLodge),
            (7, 2) => Ok(DefeatRobotsOnGuppyMound),
            (7, 3) => Ok(BeatMrsPuffsTime),
            (7, 4) => Ok(DefeatRobotsOnFlounderHill),
            (7, 5) => Ok(BeatBubbleBuddysTime),
            (7, 6) => Ok(DefeatRobotsOnSandMountain),
            (7, 7) => Ok(BeatLarrysTime),

            // Industrial Park
            (8, 0) => Ok(RoboPatrickAhoy),

            // Kelp Forest
            (9, 0) => Ok(ThroughTheWoods),
            (9, 1) => Ok(FindAllTheLostCampers),
            (9, 2) => Ok(TikiRoundup),
            (9, 3) => Ok(DownInTheSwamp),
            (9, 4) => Ok(ThroughTheKelpCaves),
            (9, 5) => Ok(PowerCrystalCrisis),
            (9, 6) => Ok(KelpVineSlide),
            (9, 7) => Ok(BeatMermaidMansTime),

            // Flying Dutchman's Graveyard
            (10, 0) => Ok(TopOfTheEntranceAreaFDG),
            (10, 1) => Ok(APathThroughTheGoo),
            (10, 2) => Ok(GooTankerAhoy),
            (10, 3) => Ok(TopOfTheStackOfShips),
            (10, 4) => Ok(ShipwreckBungee),
            (10, 5) => Ok(DestroyTheRobotShip),
            (10, 6) => Ok(GetAloftThereMatey),
            (10, 7) => Ok(DefeatTheFlyingDutchman),

            // SpongeBob's Dream
            (11, 0) => Ok(AcrossTheDreamscape),
            (11, 1) => Ok(FollowTheBouncingBall),
            (11, 2) => Ok(SlidingTexasStyle),
            (11, 3) => Ok(SwingersAhoy),
            (11, 4) => Ok(MusicIsInTheEarOfTheBeholder),
            (11, 5) => Ok(KrabbyPattyPlatforms),
            (11, 6) => Ok(SuperBounce),
            (11, 7) => Ok(HereYouGo),

            // Chum Bucket Lab,
            (12, 0) => Ok(KahRahTae),
            (12, 1) => Ok(TheSmallShallRuleOrNot),
            _ => Err(""),
        }
    }
}

impl From<Spatula> for (u32, u32) {
    fn from(spatula: Spatula) -> Self {
        use Spatula::*;
        match spatula {
            // Bikini Bottom
            OnTopOfThePineapple => (0, 0),
            OnTopOfShadyShoals => (0, 1),
            OnTopOfTheChumBucket => (0, 2),
            SpongebobsCloset => (0, 3),
            AnnoySquidward => (0, 4),
            AmbushAtTheTreeDome => (0, 5),
            InfestationAtTheKrustyKrab => (0, 6),
            AWallJumpInTheBucket => (0, 7),

            // Jellyfish Fields
            TopOfTheHill => (1, 0),
            CowaBungee => (1, 1),
            Spelunking => (1, 2),
            PatricksDilemma => (1, 3),
            NavigateTheCanyonsAndMesas => (1, 4),
            DrainTheLake => (1, 5),
            SlideLeap => (1, 6),
            DefeatKingJellyfish => (1, 7),

            // Downtown Bikini Bottom
            EndOfTheRoad => (2, 0),
            LearnSandysMoves => (2, 1),
            TikisGoBoom => (2, 2),
            AcrossTheRooftops => (2, 3),
            SwinginSandy => (2, 4),
            AmbushInTheLighthouse => (2, 5),
            ExtremeBungee => (2, 6),
            ComeBackWithTheCruiseBubble => (2, 7),

            // Goo Lagoon
            KingOfTheCastle => (3, 0),
            ConnectTheTowers => (3, 1),
            SaveTheChildren => (3, 2),
            OverTheMoat => (3, 3),
            ThroughTheSeaCaves => (3, 4),
            CleanOutTheBumperBoats => (3, 5),
            SlipAndSlideUnderThePier => (3, 6),
            TowerBungee => (3, 7),

            // Poseidome
            RumbleAtThePoseidome => (4, 0),

            // Rock Bottom
            GetToTheMuseum => (5, 0),
            SlipSlidingAway => (5, 1),
            ReturnTheMuseumsArt => (5, 2),
            SwingalongSpatula => (5, 3),
            PlunderingRobotsInTheMuseum => (5, 4),
            AcrossTheTrenchOfDarkness => (5, 5),
            LasersAreFunAndGoodForYou => (5, 6),
            HowInTarnationDoYouGetThere => (5, 7),

            // Mermalair
            TopOfTheEntranceAreaML => (6, 0),
            TopOfTheComputerArea => (6, 1),
            ShutDownTheSecuritySystem => (6, 2),
            TheFunnelMachines => (6, 3),
            TheSpinningTowersOfPower => (6, 4),
            TopOfTheSecurityTunnel => (6, 5),
            CompleteTheRollingBallRoom => (6, 6),
            DefeatPrawn => (6, 7),

            // Sand Mountain
            FrostyBungee => (7, 0),
            TopOfTheLodge => (7, 1),
            DefeatRobotsOnGuppyMound => (7, 2),
            BeatMrsPuffsTime => (7, 3),
            DefeatRobotsOnFlounderHill => (7, 4),
            BeatBubbleBuddysTime => (7, 5),
            DefeatRobotsOnSandMountain => (7, 6),
            BeatLarrysTime => (7, 7),

            // Industrial Park
            RoboPatrickAhoy => (8, 0),

            // Kelp Forest
            ThroughTheWoods => (9, 0),
            FindAllTheLostCampers => (9, 1),
            TikiRoundup => (9, 2),
            DownInTheSwamp => (9, 3),
            ThroughTheKelpCaves => (9, 4),
            PowerCrystalCrisis => (9, 5),
            KelpVineSlide => (9, 6),
            BeatMermaidMansTime => (9, 7),

            // Flying Dutchman's Graveyard
            TopOfTheEntranceAreaFDG => (10, 0),
            APathThroughTheGoo => (10, 1),
            GooTankerAhoy => (10, 2),
            TopOfTheStackOfShips => (10, 3),
            ShipwreckBungee => (10, 4),
            DestroyTheRobotShip => (10, 5),
            GetAloftThereMatey => (10, 6),
            DefeatTheFlyingDutchman => (10, 7),

            // Spongebob's Dream
            AcrossTheDreamscape => (11, 0),
            FollowTheBouncingBall => (11, 1),
            SlidingTexasStyle => (11, 2),
            SwingersAhoy => (11, 3),
            MusicIsInTheEarOfTheBeholder => (11, 4),
            KrabbyPattyPlatforms => (11, 5),
            SuperBounce => (11, 6),
            HereYouGo => (11, 7),

            // Chum Bucket Lab
            KahRahTae => (12, 0),
            TheSmallShallRuleOrNot => (12, 1),
        }
    }
}
