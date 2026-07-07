use serde::Deserialize;
use serde_json::Value;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub enum Tag {
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ModifierGroup {
    Base,
    Desecrated,
    Essence,
    Unknown,
}
#[derive(Debug)]
pub struct ModifierId(pub u16);
#[derive(Debug)]
pub struct BaseId(pub u16);
#[derive(Debug)]
pub struct ItemId(pub u16);

#[derive(Debug)]
pub struct NewBaseItem {
    pub id: ItemId,
    pub base_id: BaseId,
    pub name: String,
    pub modifiers: Vec<BaseModifier>,
}

#[derive(Debug)]
pub struct BaseModifier {
    pub id: ModifierId,
    pub group: ModifierGroup,
    pub affix: Affix,
    pub tags: Vec<Tag>,
    pub tiers: Vec<ModifierTier>,
}

#[derive(Debug)]
pub struct ModifierTier {
    pub item_level: u8,
    pub weighting: u16,
    pub values: Vec<Vec<f32>>,
}

// --- The Importer Function ---

/// Reads the file once, extracts EVERY base item into memory,
/// and immediately drops the raw JSON tree.
pub fn load_all_items(path: &str) -> Result<Vec<NewBaseItem>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let root: Value = serde_json::from_reader(file)?;

    let mut extracted_items = Vec::new();

    let bitems = match root.pointer("/bitems/seq").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return Ok(extracted_items),
    };

    // Process every single item in the array
    for bitem in bitems {
        let name = bitem["name_bitem"].as_str().unwrap_or("");

        let id = ItemId(parse_u16(&bitem["id_bitem"]).unwrap_or(0));
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

        extracted_items.push(NewBaseItem {
            id,
            base_id,
            name: name.to_string(),
            modifiers,
        });
    }

    Ok(extracted_items)
}

// --- Internal Helper Functions ---

fn build_modifier(root: &Value, mod_id_str: &str, base_id_str: &str) -> Option<BaseModifier> {
    let mod_seq = root.pointer("/modifiers/seq")?.as_array()?;
    let mod_data = mod_seq.iter().find(|m| {
        parse_u16(&m["id_modifier"]).map(|id| id.to_string()) == Some(mod_id_str.to_string())
    })?;

    let id = ModifierId(mod_id_str.parse().ok()?);
    let affix = Affix::from(mod_data["affix"].as_str().unwrap_or(""));

    let mgroup_str = mod_data["id_mgroup"].as_str().unwrap();

    let group = if mgroup_str == "10" {
        ModifierGroup::Desecrated
    } else if mgroup_str == "13" {
        ModifierGroup::Essence
    } else {
        ModifierGroup::Base
    };

    let tags: Vec<Tag> =
        serde_json::from_str(mod_data["modgroups"].as_str().unwrap_or("[]")).unwrap_or_default();

    let mut tiers = Vec::new();
    if let Some(tier_list) = root
        .pointer(&format!("/tiers/{}/{}", mod_id_str, base_id_str))
        .and_then(|v| v.as_array())
    {
        for tier_val in tier_list {
            let item_level = parse_u16(&tier_val["ilvl"]).unwrap_or(0) as u8;
            let weighting = parse_u16(&tier_val["weighting"]).unwrap_or(0);

            let mut values = Vec::new();
            if let Some(nval_str) = tier_val["nvalues"].as_str() {
                if let Ok(parsed) = serde_json::from_str::<Vec<Vec<f32>>>(nval_str) {
                    values = parsed;
                }
            }

            tiers.push(ModifierTier {
                item_level,
                weighting,
                values,
            });
        }
    }

    Some(BaseModifier {
        id,
        group,
        affix,
        tiers,
        tags,
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
