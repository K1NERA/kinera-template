use cumulus_primitives_core::ParaId;
use kine_runtime::{
	AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT,
	KineTagsConfig, MaxTags, 
	CategoryStringLimit, TagStringLimit,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify, BlockNumberProvider},
	BoundedVec,
};



/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> kine_runtime::SessionKeys {
	kine_runtime::SessionKeys { aura: keys }
}




pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		kine_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(),
			// You MUST set this to the correct network!
			para_id: 1000,
		},
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed("Bob"),
			),
		],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		1000.into(),
	))
	.build()
}




pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	#[allow(deprecated)]
	ChainSpec::builder(
		kine_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(),
			// You MUST set this to the correct network!
			para_id: 1000,
		},
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed("Bob"),
			),
		],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		1000.into(),
	))
	.with_protocol_id("template-local")
	.with_properties(properties)
	.build()
}




fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root: AccountId,
	id: ParaId,
) -> serde_json::Value {



	// Tags Ranking Lists

	let rl_cinema_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
	= vec![
		"Fiction".as_bytes().to_vec().try_into().unwrap(),
		"Science Fiction".as_bytes().to_vec().try_into().unwrap(),
		"Drama".as_bytes().to_vec().try_into().unwrap(),
		"Documentary".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let rl_stars_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Influencers".as_bytes().to_vec().try_into().unwrap(),
			"Educator".as_bytes().to_vec().try_into().unwrap(),
			"Pivots".as_bytes().to_vec().try_into().unwrap(),
			"Web3".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let rl_just_fun_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Art".as_bytes().to_vec().try_into().unwrap(),
			"Dance".as_bytes().to_vec().try_into().unwrap(),
			"Disasters".as_bytes().to_vec().try_into().unwrap(),
			"Kids".as_bytes().to_vec().try_into().unwrap(),
			"Sports".as_bytes().to_vec().try_into().unwrap(),
			"Products".as_bytes().to_vec().try_into().unwrap(),
			"Professionals".as_bytes().to_vec().try_into().unwrap(),
			"News".as_bytes().to_vec().try_into().unwrap(),
			"Services".as_bytes().to_vec().try_into().unwrap(),
			"Just FUN Others".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();
		
	let rl_we_festivals_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Information".as_bytes().to_vec().try_into().unwrap(),
			"Offbeat Films/Marginal".as_bytes().to_vec().try_into().unwrap(),
			"Experimental Cinema".as_bytes().to_vec().try_into().unwrap(),
			"Video Art".as_bytes().to_vec().try_into().unwrap(),
			"Video Clips".as_bytes().to_vec().try_into().unwrap(),
			"We Festivals Others".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();
		
	let rl_artificial_intelligence_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"AI Drama".as_bytes().to_vec().try_into().unwrap(),
			"AI Documentary".as_bytes().to_vec().try_into().unwrap(),
			"AI Fiction".as_bytes().to_vec().try_into().unwrap(),
			"AI Fake".as_bytes().to_vec().try_into().unwrap(),
			"AI Science Fiction".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let rl_gaming_streamers_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Action/Adventure".as_bytes().to_vec().try_into().unwrap(),
			"Role-Playing".as_bytes().to_vec().try_into().unwrap(),
			"Strategy".as_bytes().to_vec().try_into().unwrap(),
			"Sports".as_bytes().to_vec().try_into().unwrap(),
			"Puzzle".as_bytes().to_vec().try_into().unwrap(),
			"Simulation".as_bytes().to_vec().try_into().unwrap(),
			"Racing".as_bytes().to_vec().try_into().unwrap(),
			"Fighting".as_bytes().to_vec().try_into().unwrap(),
			"Platformer".as_bytes().to_vec().try_into().unwrap(),
			"MMO (Massive Multiplayer Online)".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();



	// Tags Moderation

	let mod_violence_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_discrimination_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_lack_of_consent_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_impersonation_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_terrorism_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_copyright_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_misinformation_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_pornography_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_extreme_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_naming_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();

	let mod_categorization_tags : BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
		= vec![
			"Movie".as_bytes().to_vec().try_into().unwrap(),
			"Festival".as_bytes().to_vec().try_into().unwrap(),
			"RankingList".as_bytes().to_vec().try_into().unwrap(),
	].try_into().unwrap();




	// setup the final map with all categories and tags by type
	let initial_categories_and_tags: Vec <(
		(BoundedVec<u8, CategoryStringLimit>, BoundedVec<u8, CategoryStringLimit>),
		BoundedVec<BoundedVec<u8, TagStringLimit>, MaxTags>
	)> = vec![
		(("Movie".as_bytes().to_vec().try_into().unwrap(), "Cinema".as_bytes().to_vec().try_into().unwrap()), rl_cinema_tags.clone()),
		(("Movie".as_bytes().to_vec().try_into().unwrap(), "Stars".as_bytes().to_vec().try_into().unwrap()), rl_stars_tags.clone()),
		(("Movie".as_bytes().to_vec().try_into().unwrap(), "Just FUN".as_bytes().to_vec().try_into().unwrap()), rl_just_fun_tags.clone()),
		(("Movie".as_bytes().to_vec().try_into().unwrap(), "We Festivals".as_bytes().to_vec().try_into().unwrap()), rl_we_festivals_tags.clone()),
		(("Movie".as_bytes().to_vec().try_into().unwrap(), "Artifitial Intelligence".as_bytes().to_vec().try_into().unwrap()), rl_artificial_intelligence_tags.clone()),
		(("Movie".as_bytes().to_vec().try_into().unwrap(), "Gaming/Streamers".as_bytes().to_vec().try_into().unwrap()), rl_gaming_streamers_tags.clone()),

		(("Festival".as_bytes().to_vec().try_into().unwrap(), "Cinema".as_bytes().to_vec().try_into().unwrap()), rl_cinema_tags.clone()),
		(("Festival".as_bytes().to_vec().try_into().unwrap(), "Stars".as_bytes().to_vec().try_into().unwrap()), rl_stars_tags.clone()),
		(("Festival".as_bytes().to_vec().try_into().unwrap(), "Just FUN".as_bytes().to_vec().try_into().unwrap()), rl_just_fun_tags.clone()),
		(("Festival".as_bytes().to_vec().try_into().unwrap(), "We Festivals".as_bytes().to_vec().try_into().unwrap()), rl_we_festivals_tags.clone()),
		(("Festival".as_bytes().to_vec().try_into().unwrap(), "Artifitial Intelligence".as_bytes().to_vec().try_into().unwrap()), rl_artificial_intelligence_tags.clone()),
		(("Festival".as_bytes().to_vec().try_into().unwrap(), "Gaming/Streamers".as_bytes().to_vec().try_into().unwrap()), rl_gaming_streamers_tags.clone()),

		(("Ranking List".as_bytes().to_vec().try_into().unwrap(), "Cinema".as_bytes().to_vec().try_into().unwrap()), rl_cinema_tags),
		(("Ranking List".as_bytes().to_vec().try_into().unwrap(), "Stars".as_bytes().to_vec().try_into().unwrap()), rl_stars_tags),
		(("Ranking List".as_bytes().to_vec().try_into().unwrap(), "Just FUN".as_bytes().to_vec().try_into().unwrap()), rl_just_fun_tags),
		(("Ranking List".as_bytes().to_vec().try_into().unwrap(), "We Festivals".as_bytes().to_vec().try_into().unwrap()), rl_we_festivals_tags),
		(("Ranking List".as_bytes().to_vec().try_into().unwrap(), "Artifitial Intelligence".as_bytes().to_vec().try_into().unwrap()), rl_artificial_intelligence_tags),
		(("Ranking List".as_bytes().to_vec().try_into().unwrap(), "Gaming/Streamers".as_bytes().to_vec().try_into().unwrap()), rl_gaming_streamers_tags),

		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Violence".as_bytes().to_vec().try_into().unwrap()), mod_violence_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Discrimination".as_bytes().to_vec().try_into().unwrap()), mod_discrimination_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "LackOfConsent".as_bytes().to_vec().try_into().unwrap()), mod_lack_of_consent_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Impersonation".as_bytes().to_vec().try_into().unwrap()), mod_impersonation_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Terrorism".as_bytes().to_vec().try_into().unwrap()), mod_terrorism_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Copyright".as_bytes().to_vec().try_into().unwrap()), mod_copyright_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Misinformation".as_bytes().to_vec().try_into().unwrap()), mod_misinformation_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Pornography".as_bytes().to_vec().try_into().unwrap()), mod_pornography_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Extreme".as_bytes().to_vec().try_into().unwrap()), mod_extreme_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Naming".as_bytes().to_vec().try_into().unwrap()), mod_naming_tags),
		(("Moderation".as_bytes().to_vec().try_into().unwrap(), "Categorization".as_bytes().to_vec().try_into().unwrap()), mod_categorization_tags),
	];



	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
			.collect::<Vec<_>>(),
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
		"sudo": { "key": Some(root) },

		"kineTags": {
			"categoryToTagMap": initial_categories_and_tags.iter().cloned().map(|x| x).collect::<Vec<_>>(),
		},
	})
}
