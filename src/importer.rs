use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, path::Path};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ModifierFamily {
    Abyss4AdditionalChance,
    AbyssAdditionalChance,
    AbyssDepthsChance,
    AbyssEnhancedMonstersPerChasm,
    AbyssExtraModifiers,
    AbyssExtraTickets,
    AbyssIncreasedRewards,
    AbyssJewelMod,
    AbyssMonsterIncrease,
    AbyssRareMonsterIncrease,
    AdditionalAmmo,
    AdditionalArrows,
    AdditionalBallistaTotem,
    AdditionalBlock,
    AdditionalCharm,
    AdditionalComboChance,
    AdditionalProjectiles,
    AdditionalSpellTotem,
    AdditionalTotems,
    AftershockChance,
    AilmentChance,
    AilmentEffect,
    AilmentThreshold,
    AilmentThresholdfromEnergyShield,
    AllAttributes,
    AllDamage,
    AllDefences,
    AllResistances,
    ArcaneSurgeEffect,
    ArcaneSurgeOnCrit,
    ArchonDelayRecovery,
    ArchonDuration,
    AreaOfEffect,
    ArmourAppliesToChaosDamage,
    ArmourAppliesToElementalDamage,
    ArmourBreak,
    ArmourBreakDuration,
    ArmourPenetration,
    AttackDamage,
    AuraEffect,
    BannerArea,
    BannerDuration,
    BannerValourGained,
    BaseCurseDuration,
    BaseLocalDefences,
    BaseLocalDefencesAndDefencePercent,
    BaseLocalDefencesAndLife,
    BaseLocalDefencesAndMana,
    BaseRunicWard,
    BaseSpirit,
    BellHitLimit,
    BeltCharmChargesGained,
    BeltCharmChargesUsed,
    BeltCharmDuration,
    BeltFlaskChargesGained,
    BeltFlaskChargesUsed,
    BeltFlaskRecoveryRate,
    BleedChanceIncrease,
    BleedDuration,
    BleedingDamage,
    BlindEffect,
    BlindOnHit,
    BlindingHit,
    BlockDamageTakenWhileActiveBlocking,
    BodyArmourFromBodyArmour,
    Breach3AdditionalChance,
    BreachAdditionalChance,
    BreachAdditionalRares,
    BreachBossChance,
    BreachChestAdditional,
    BreachHivebloodQuantity,
    BreachMagicMonsterIncrease,
    BreachMonsterQuantity,
    BreachMonsterSplinterIncrease,
    BreachRareMonsterIncrease,
    BreachRareMonsterPotency,
    BreachSpeedIncrease,
    BreachWombgiftQuantity,
    CastSpeedOnFullLife,
    CastSpeedOnLowLife,
    Chain,
    ChainFromTerrain,
    ChanceToGainAnAdditionalInfusion,
    ChanceToGainNaturesArchon,
    ChanceToPierce,
    ChanceToTakeCriticalStrike,
    ChaosResistance,
    CharmChargesGained,
    CharmChargesUsed,
    CharmDamageWhileUsing,
    CharmDuration,
    ColdAndChaosDamageResistance,
    ColdDamage,
    ColdDamagePercentage,
    ColdDamagePercentagePrefix,
    ColdDamageTakenRecoupedAsLife,
    ColdResistance,
    ColdResistancePenetration,
    CompanionAttackSpeed,
    CompanionDamage,
    CompanionLife,
    CompanionReservationEfficiency,
    CooldownRecovery,
    CorruptedBlood,
    CorruptionIntertactions,
    CriticalAilmentEffect,
    CriticalStrikeChanceIncrease,
    CriticalStrikeMultiplier,
    CrossbowDamage,
    CrossbowReloadSpeed,
    CrossbowSpeed,
    CurseAreaOfEffect,
    CurseCastSpeed,
    CurseDelay,
    CurseEffectiveness,
    DamageForm,
    DamageRemovedFromManaBeforeLife,
    DamageTakenGainedAsLife,
    DamageVsRareOrUnique,
    DamageWithTriggeredSpells,
    DamageWithWeaponTypeSkill,
    DamagevsArmourBrokenEnemies,
    DamagingAilmentDuration,
    DazeBuildup,
    DebuffTime,
    DefencesPercent,
    DefencesPercentAndStunThreshold,
    DeflectDamageTaken,
    DeliriumAdditionalRewardType,
    DeliriumBossChance,
    DeliriumDifficultyIncrease,
    DeliriumDoodadsIncrease,
    DeliriumFogDissipationDelay,
    DeliriumFogPersistence,
    DeliriumMonsterSplinterIncrease,
    DeliriumPackSizeIncrease,
    DeliriumRareMonsterPause,
    DeliriumRewardProgressIncrease,
    Dexterity,
    DodgeRoll,
    ElementalAilmentDuration,
    ElementalAilmentEffect,
    ElementalDamagePercent,
    ElementalDamageTakenRecoupedAsEnergyShield,
    ElementalInfusion,
    ElementalPenetration,
    ElementalSkillLimit,
    EnchantmentHeistArmour,
    Energy,
    EnergyShieldDelay,
    EnergyShieldPercent,
    EnergyShieldRechargeRateIfBlockedRecently,
    EnergyShieldRegeneration,
    EquipmentModifierEffect,
    EssenceAbyss,
    EssenceGoldDropped,
    EssenceGrantedPassive,
    EvasionAppliesToDeflection,
    EvasionRatingFromBodyArmour,
    EvasionRatingPercent,
    ExertedAttackDamage,
    ExpeditionArtifactIncrease,
    ExpeditionExplosionPlacement,
    ExpeditionExplosionRadius,
    ExpeditionLogbookIncrease,
    ExpeditionRareMonsters,
    ExpeditionRelicIncrease,
    ExpeditionRelicModEffect,
    ExpeditionRunicMonsters,
    ExposureEffect,
    FasterAilmentDamage,
    FireAndChaosDamageResistance,
    FireDamage,
    FireDamagePercentage,
    FireDamagePercentagePrefix,
    FireDamageTakenRecoupedAsLife,
    FireResistance,
    FireResistancePenetration,
    FissureChance,
    FlaskBuffWhileHealing,
    FlaskChargeGeneration,
    FlaskChargeGenerationPercent,
    FlaskChargesUsed,
    FlaskDuration,
    FlaskExtraLifeRegeneration,
    FlaskGainCharge,
    FlaskHealsOthers,
    FlaskLifeRecovery,
    FlaskManaRecovery,
    FlaskNumCharges,
    FlaskRechargeRate,
    FlaskRecoveryAmount,
    FlaskRecoverySpeed,
    FocusEnergyShield,
    ForkingProjectiles,
    FormCritMultiplier,
    FreezeDamageIncrease,
    FreezeThreshold,
    GainFlatLifeOnBlock,
    GainLifeOnBlock,
    GainManaOnBlock,
    GainRage,
    GlobalDeflectionRating,
    GlobalItemAttributeRequirements,
    GlobalSkillGemQuality,
    GloryChanceToNotConsume,
    GloryGeneration,
    GrenadeAdditionalDetonationChance,
    GrenadeCooldownUse,
    GrenadeDamage,
    GrenadeDuration,
    HazardDamage,
    HeavyStunDecayRate,
    HeraldDamage,
    HeraldReservationEfficiency,
    HinderedEnemyTakeIncreasedDamage,
    IgniteChanceIncrease,
    IgniteEffect,
    Immobilise,
    ImmuneToMaim,
    ImmunityToBlind,
    IncisionChance,
    IncreaseSocketedGemLevel,
    IncreasedAccuracy,
    IncreasedAccuracyPercent,
    IncreasedAttackAreaOfEffect,
    IncreasedAttackSpeed,
    IncreasedBlockChance,
    IncreasedCastSpeed,
    IncreasedChaosDamage,
    IncreasedChillDuration,
    IncreasedEnergyShield,
    IncreasedEvasionRating,
    IncreasedLife,
    IncreasedMana,
    IncreasedMaximumPowerCharges,
    IncreasedMinionDamageIfYouHitEnemy,
    IncreasedPhysicalDamageReductionRating,
    IncreasedPhysicalDamageReductionRatingPercent,
    IncreasedShieldBlockPercentage,
    IncreasedSpeed,
    IncreasedSpellDamageOnFullEnergyShield,
    IncreasedStunThreshold,
    IncreasedStunThresholdIfNoRecentStun,
    IncreasedTotemLife,
    IncreasedWeaponElementalDamagePercent,
    IncursionSecondaryEncounters,
    InstantLeechPercent,
    Intelligence,
    InvocatedSpellHalfEnergyChance,
    ItemFoundRarityIncrease,
    ItemFoundRarityIncreasePrefix,
    Knockback,
    LifeCost,
    LifeCostEfficiency,
    LifeGainPerTarget,
    LifeGainedFromEnemyDeath,
    LifeLeech,
    LifeRegeneration,
    LifeRegenerationPlusPercentWhileMoving,
    LifeRegenerationRate,
    LightRadiusAndAccuracy,
    LightRadiusAndManaRegeneration,
    LightningAndChaosDamageResistance,
    LightningDamage,
    LightningDamageCanIgnite,
    LightningDamagePercentage,
    LightningDamagePercentagePrefix,
    LightningDamageTakenRecoupedAsLife,
    LightningResistance,
    LightningResistancePenetration,
    LocalAttributeRequirements,
    LocalChanceToBleed,
    LocalChanceToPoisonOnHit,
    LocalChaosDamage,
    LocalIncreasedPhysicalDamagePercentAndAccuracyRating,
    LocalIncreasedSpiritAndMana,
    LocalIncreasedSpiritPercent,
    LocalMaimOnHit,
    LocalMaximumQuality,
    LocalPhysicalDamagePercent,
    LocalRunicWardPercent,
    MaceStun,
    ManaCostEfficiency,
    ManaGainedFromEnemyDeath,
    ManaGainedOnKillPercentage,
    ManaLeech,
    ManaLeechAmount,
    ManaRegeneration,
    MapAdditionalAzmeriWisp,
    MapAdditionalChests,
    MapAdditionalEssence,
    MapAdditionalExile,
    MapAdditionalModifier,
    MapAdditionalShrine,
    MapAdditionalSpirit,
    MapAdditionalStoneCircle,
    MapAdditionalStrongbox,
    MapAdditionalUniqueMonsterModifier,
    MapBleeding,
    MapBossAdditionalEssence,
    MapBossAdditionalShrine,
    MapBossAdditionalSpirit,
    MapBossAdditionalStrongbox,
    MapBossExperience,
    MapBossQuantity,
    MapBossRarity,
    MapBossWaystoneChance,
    MapChestCountIncrease,
    MapDroppedGoldIncrease,
    MapDroppedItemQuantityIncrease,
    MapDroppedItemQuantityRarityIncrease,
    MapDroppedItemRarityIncrease,
    MapDroppedMapsIncrease,
    MapExperienceGainIncrease,
    MapMagicChestCountIncrease,
    MapMagicPackIncrease,
    MapMagicPackSizeIncrease,
    MapMonsterAdditionalPacks,
    MapMonsterAreaOfEffect,
    MapMonsterChaosDamage,
    MapMonsterColdDamage,
    MapMonsterCriticalStrikesAndDamage,
    MapMonsterDamage,
    MapMonsterEffectiveness,
    MapMonsterFast,
    MapMonsterFireDamage,
    MapMonsterLife,
    MapMonsterLightningDamage,
    MapMonsterMultipleProjectiles,
    MapMonsterRarityIncrease,
    MapMonstersAccuracy,
    MapMonstersAilmentChance,
    MapMonstersAllResistances,
    MapMonstersArmourBreak,
    MapMonstersArmoured,
    MapMonstersBaseSelfCriticalMultiplier,
    MapMonstersCurseEffectOnSelfFinal,
    MapMonstersElementalPenetration,
    MapMonstersEnergyShield,
    MapMonstersEvasive,
    MapMonstersStealChargesOnHit,
    MapMonstersStunAndAilmentThreshold,
    MapMonstersStunBuildup,
    MapPackSizeIncrease,
    MapPlayerCooldownRecovery,
    MapPlayerElementalWeakness,
    MapPlayerEnfeeblement,
    MapPlayerMaxResists,
    MapPlayerReducedRegen,
    MapPlayerTemporalChains,
    MapPlayersGainReducedFlaskCharges,
    MapPoisoning,
    MapRareChestCountIncrease,
    MapRareMonstersAdditionalModifier,
    MapRarePackIncrease,
    MapSpreadGroundEffect,
    MapTotalEffectivenessIncrease,
    MarkCritical,
    MarkDamage,
    MarkDuration,
    MartialWeaponGainedDamage,
    MaximumBlockChance,
    MaximumChaosResistance,
    MaximumColdResist,
    MaximumEnduranceCharges,
    MaximumEnergyShieldFromBodyArmour,
    MaximumFireResist,
    MaximumFrenzyCharges,
    MaximumLifeIncreasePercent,
    MaximumLifeOnKillPercent,
    MaximumLightningResistance,
    MaximumManaIncreasePercent,
    MaximumPuppeteerStacks,
    MaximumRage,
    MaximumResistances,
    MeleeDamage,
    MeleeDamageIfProjectileHitRecently,
    MeleeWeaponAndUnarmedRange,
    MinionAccuracyRating,
    MinionAreaOfEffect,
    MinionAttackSpeedAndCastSpeed,
    MinionChaosResistance,
    MinionCooldown,
    MinionCriticalStrikeChance,
    MinionCriticalStrikeMultiplier,
    MinionDamage,
    MinionDamagingAilments,
    MinionElementalResistances,
    MinionGainPuppetMasterOnCommand,
    MinionLife,
    MinionPhysicalDamageReduction,
    MovementVelocity,
    OfferingDuration,
    OfferingEffect,
    OfferingLife,
    Onslaught,
    ParriedDebuff,
    PercentDamageGoesToMana,
    PercentageDexterity,
    PercentageIntelligence,
    PercentageStrength,
    PhysicalDamage,
    PhysicalDamagePercent,
    PhysicalDamageTakenAsChaos,
    PhysicalDamageTakenAsLightningWhileActiveBlocking,
    PinBuildup,
    PoisonChanceIncrease,
    PoisonDuration,
    PoisonEffect,
    PresenceRadius,
    ProjectileDamage,
    ProjectileDamageIfMeleeHitRecently,
    ProjectileSpeed,
    QuarterstaffFreezeBuildup,
    RageOnHit,
    RecoverPercentMaxLifeOnKill,
    RecoverResourceOnExpendingCombo,
    RecoverRunicWardOnCharmUse,
    ReducedAilmentDuration,
    ReducedCriticalStrikeDamageTaken,
    ReducedCurseEffect,
    ReducedExtraDamageFromCrits,
    ReducedPhysicalDamageTaken,
    ReducedSelfIgniteDuration,
    RemnantEffect,
    RemnantPickupRadius,
    RitualAdditionalReroll,
    RitualChanceForNoCost,
    RitualDeferCostIncrease,
    RitualDeferSpeed,
    RitualMagicMonsters,
    RitualOmenChance,
    RitualRareMonsters,
    RitualRerollCostIncrease,
    RitualTributeIncrease,
    RunicWardPercent,
    ShieldArmourIncrease,
    ShieldSkillsStunningAlsoFullyBreaksArmour,
    ShockChanceIncrease,
    ShockDuration,
    ShockEffect,
    SkillCostEfficiency,
    SkillEffectDuration,
    SkillLifeCost,
    SlowEffect,
    SlowPotency,
    SoulCore,
    SpearAttackSpeed,
    SpearCriticalDamage,
    SpearDamage,
    SpecificWeaponAccuracy,
    SpecificWeaponDamage,
    SpecificWeaponSpeed,
    SpellAreaOfEffect,
    SpellCriticalStrikeChanceIncrease,
    SpellDamage,
    SpellDamageAndMana,
    SpellsFire2AdditionalProjectileChance,
    SpiritReservationEfficiency,
    Strength,
    StunDamageIncrease,
    StunDurationIncreasePercent,
    StunThreshold,
    StunThresholdDuringParry,
    StunThresholdfromEnergyShield,
    SummonTotemCastSpeed,
    TemporaryMinionLimit,
    Thorns,
    TotemDamage,
    WarcriesExertAnAdditionalAttack,
    WarcryCooldownSpeed,
    WarcryDamage,
    WarcryEffect,
    WarcrySpeed,
    WardRegenerationRate,
    WeaponCasterDamagePrefix,
    WeaponDamageTypePrefix,
    WeaponSwapSpeed,
    WitheredEffect,
    YouCannotBeHindered,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Affix {
    Prefix,
    Suffix,
    Socket,
    Corrupted,
    Unknown(String),
}

impl From<&str> for Affix {
    fn from(s: &str) -> Self {
        match s {
            "prefix" => Affix::Prefix,
            "suffix" => Affix::Suffix,
            "socket" => Affix::Socket,
            "corrupted" => Affix::Corrupted,
            other => Affix::Unknown(other.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModifierType {
    Base,
    Desecrated,
    Essence,
    Unknown,
}
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ModifierId(pub u16);
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BaseId(pub u16);
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BaseItemId(pub u16);
#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SocketableId(pub u16);

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseItem {
    pub id: BaseItemId,
    pub base_id: BaseId,
    pub modifier_definitions: Vec<ModifierDefinition>,
    pub name: String,
    pub base_name: String,
    pub is_jewellery: bool,
    pub is_martial: bool,

    // TODO
    // pub socketables: Vec<Socketable>

    // Base group stuff
    pub item_group_name: String, // "Body Armours", "Gloves", etc.
    pub max_affix: u16,          // Maximum number of explicit affixes
    pub can_use_essence: bool,   // Essences can be applied to this item group.
    pub can_use_catalyst: bool,  // Catalysts can be applied.
    pub max_sockets: u16,        // Maximum socket count.

    // useless stuff
    pub can_spawn_rare: bool, // Whether this item group can naturally exist as Rare.

    pub is_influenced: bool, // Legacy PoE1 flag.
    pub is_fossil: bool,     // Legacy PoE1 flag.
    pub is_craftable: bool,  // Legacy PoE1 flag.
    pub has_items: bool,     // idk
    pub is_notable: bool,    // maybe legacy?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifierDefinition {
    pub id: ModifierId,
    pub name: String,
    pub modifier_type: ModifierType,
    pub affix: Affix,
    pub modifier_families: Vec<ModifierFamily>,
    pub modifier_tags: Vec<ModifierTag>,
    pub tiers: Vec<ModifierTier>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierTag {
    Ailment,
    AmanamuMod,
    Armour,
    Attack,
    Attribute,
    Aura,
    Bleed,
    Caster,
    CasterCritical,
    CasterSpeed,
    JewelleryAttack,
    JewelleryAttribute,
    JewelleryCaster,
    JewelleryDefense,
    JewelleryElemental,
    JewelleryResistance,
    JewelleryResource,
    Chaos,
    ChaosDamage,
    ChaosResistance,
    Cold,
    ColdResistance,
    Critical,
    Curse,
    Damage,
    Defences,
    Drop,
    Elemental,
    ElementalResistance,
    EnergyShield,
    Evasion,
    Fire,
    FireResistance,
    Gem,
    GemLevel,
    HasAttackMod,
    KurgalMod,
    Life,
    FlatLifeRegen,
    Lightning,
    LightningResistance,
    Mana,
    Minion,
    MinionDamage,
    MinionResistance,
    MinionSpeed,
    Physical,
    PhysicalDamage,
    Poison,
    Resistance,
    RunicWard,
    Speed,
    UlamanMod,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RollableValue {
    Between((f32, f32)),
    Fixed(f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifierTier {
    pub tier: u8,
    pub item_level: u8,
    pub weighting: u16,
    pub values: Vec<RollableValue>,
}

#[derive(Serialize, Deserialize)]
pub enum SocketType {
    Rune,
    SoulCore,
    Talisman,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socketable {
    pub id: SocketableId,
    pub name: String, // TODO: info
}

#[derive(Serialize, Deserialize)]
pub struct PoeData {
    pub base_items: Vec<BaseItem>,
}

pub fn load_poe_data_cached(path: &str) -> Result<PoeData, Box<dyn std::error::Error>> {
    let bin_path = Path::new(path).with_extension("bin");

    if bin_path.exists() {
        let bytes = std::fs::read(&bin_path)?;
        let data = postcard::from_bytes(&bytes)?;
        return Ok(data);
    }

    let data = load_poe_data(path)?;

    let bytes = postcard::to_allocvec(&data)?;
    std::fs::write(&bin_path, bytes)?;

    Ok(data)
}

fn load_poe_data(path: &str) -> Result<PoeData, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let root: Value = serde_json::from_reader(file)?;

    let mut base_items = Vec::new();

    let bitems = root
        .pointer("/bitems/seq")
        .and_then(|v| v.as_array())
        .expect("/bitems/seq to exist and be an array.");

    let bases = root
        .pointer("/bases/seq")
        .and_then(|v| v.as_array())
        .expect("/bases/seq to exist and be an array.");

    let bgroups = root
        .pointer("/bgroups/seq")
        .and_then(|v| v.as_array())
        .expect("/bgroups/seq to exist and be an array.");

    for bitem in bitems {
        let id = BaseItemId(parse_u16(&bitem["id_bitem"]).unwrap_or(0));
        let base_id = BaseId(parse_u16(&bitem["id_base"]).unwrap_or(0));
        let base_id_str = base_id.0.to_string();

        let mut modifiers = Vec::new();
        if let Some(mod_id_list) = root
            .pointer(&format!("/basemods/{}", base_id_str))
            .and_then(|v| v.as_array())
        {
            for mod_val in mod_id_list {
                if let Some(mod_id_str) = mod_val.as_str() {
                    if let Some(modifier) = build_modifier(&root, mod_id_str, &base_id_str) {
                        modifiers.push(modifier);
                    }
                }
            }
        }

        if let Some(base) = bases
            .iter()
            .find(|b| b["id_base"].as_str().unwrap() == bitem["id_base"].as_str().unwrap())
        {
            if let Some(base_group) = bgroups
                .iter()
                .find(|bg| bg["id_bgroup"].as_str().unwrap() == base["id_bgroup"])
            {
                base_items.push(BaseItem {
                    id,
                    base_id,
                    modifier_definitions: modifiers,
                    name: bitem["name_bitem"].as_str().unwrap().to_string(),
                    base_name: base["name_base"].as_str().unwrap().to_string(),
                    is_jewellery: base["is_jewellery"] == "1",
                    is_martial: base["is_martial"] == "1",
                    item_group_name: base_group["name_bgroup"].as_str().unwrap().to_string(),
                    max_affix: parse_u16(&base_group["max_affix"]).unwrap_or(0),
                    can_spawn_rare: base_group["is_rare"] == "1",
                    is_influenced: base_group["is_influenced"] == "1",
                    is_fossil: base_group["is_fossil"] == "1",
                    can_use_essence: base_group["is_ess"] == "1",
                    is_craftable: base_group["is_craftable"] == "1",
                    is_notable: base_group["is_notable"] == "1",
                    can_use_catalyst: base_group["is_catalyst"] == "1",
                    has_items: base_group["has_items"] == "1",
                    max_sockets: parse_u16(&base_group["max_sockets"]).unwrap_or(0),
                });
            }
        }
    }

    Ok(PoeData { base_items })
}

// --- Internal Helper Functions ---

fn build_modifier(root: &Value, mod_id_str: &str, base_id_str: &str) -> Option<ModifierDefinition> {
    let mod_seq = root.pointer("/modifiers/seq")?.as_array()?;

    let mtypes = root
        .pointer("/mtypes/seq")
        .and_then(|v| v.as_array())
        .expect("/mtypes/seq to exist and be an array.");

    let mod_data = mod_seq.iter().find(|m| {
        parse_u16(&m["id_modifier"]).map(|id| id.to_string()) == Some(mod_id_str.to_string())
    })?;

    let id = ModifierId(mod_id_str.parse().ok()?);
    let affix = Affix::from(mod_data["affix"].as_str().unwrap_or(""));
    let name = mod_data["name_modifier"].as_str().unwrap().to_string();

    let mgroup_str = mod_data["id_mgroup"].as_str().unwrap();

    let mod_types = mod_data["mtypes"].as_str().map(|s| {
        s.split('|')
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>()
    });

    let modifier_tags = mod_types
        .map(|mtttt| {
            mtttt
                .iter()
                .map(|mt| {
                    let mod_type_data = mtypes
                        .iter()
                        .find(|modt| modt["id_mtype"].as_str().unwrap() == mt)
                        .unwrap();
                    let tag: ModifierTag =
                        serde_json::from_value(mod_type_data["poedb_id"].clone()).unwrap();
                    tag
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let group = if mgroup_str == "10" {
        ModifierType::Desecrated
    } else if mgroup_str == "13" {
        ModifierType::Essence
    } else {
        ModifierType::Base
    };

    let modifier_families: Vec<ModifierFamily> =
        serde_json::from_str(mod_data["modgroups"].as_str().unwrap_or("[]")).unwrap_or_default();

    let mut tiers = Vec::new();
    if let Some(tier_list) = root
        .pointer(&format!("/tiers/{}/{}", mod_id_str, base_id_str))
        .and_then(|v| v.as_array())
    {
        for (i, tier_val) in tier_list.iter().enumerate() {
            let item_level = parse_u16(&tier_val["ilvl"]).unwrap_or(0) as u8;
            let weighting = parse_u16(&tier_val["weighting"]).unwrap_or(0);

            let mut values = Vec::<RollableValue>::new();
            if let Some(nval_str) = tier_val["nvalues"].as_str() {
                if let Ok(parsed) = serde_json::from_str::<Vec<Value>>(nval_str) {
                    for v in parsed.into_iter() {
                        if let Ok(fixed) = serde_json::from_value::<f32>(v.clone()) {
                            values.push(RollableValue::Fixed(fixed))
                        } else if let Ok(range) = serde_json::from_value::<(f32, f32)>(v) {
                            values.push(RollableValue::Between(range))
                        } else {
                            // bad!
                        }
                    }
                }
            }

            tiers.push(ModifierTier {
                item_level,
                weighting,
                values,
                tier: (tier_list.len() - i) as u8,
            });
        }
    }

    Some(ModifierDefinition {
        id,
        modifier_type: group,
        affix,
        tiers,
        modifier_families,
        name,
        modifier_tags,
    })
}

fn parse_u16(val: &Value) -> Option<u16> {
    if let Some(n) = val.as_u64() {
        Some(n as u16)
    } else if let Some(s) = val.as_str() {
        s.parse().ok()
    } else {
        None
    }
}
