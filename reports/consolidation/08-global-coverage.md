# Phase 2 — Step 4 Prep (cont.): Global Slot Coverage Report

This report documents the global slot coverage analysis across all DAG taxonomies and constellation maps, ignoring pack boundaries.

---

## Summary Metrics
* **GLOBAL_DAG Unique Slot Count**: 120
* **GLOBAL_CMAP Unique Slot Count**: 124

---

## ORPHAN-RICH (Keys in GLOBAL_CMAP but in NO DAG)
These slots define rich database mappings or verb palettes in constellation maps, but have no corresponding DAG lifecycle transitions to gate them. This is the critical direction indicating un-gated action surfaces.

* **`access_review`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`acs_operator`**
  * Defined in: [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml)
  * Status: Declares Verb Palette
* **`administrator`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml), [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml), [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml), [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`aggregator`**
  * Defined in: [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml)
  * Status: Declares Verb Palette
* **`aifm`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml)
  * Status: Declares Verb Palette
* **`auditor`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml), [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml), [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml), [struct_uk_manager_llp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_manager_llp.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml), [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`authorised_corporate_director`**
  * Defined in: [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml)
  * Status: Declares Verb Palette
* **`authorised_fund_manager`**
  * Defined in: [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml)
  * Status: Declares Verb Palette
* **`authorized_participant`**
  * Defined in: [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml)
  * Status: Declares Verb Palette
* **`bods`**
  * Defined in: [kyc_extended.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_extended.yaml)
  * Status: Declares Verb Palette
* **`case`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml), [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml), [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml), [struct_uk_manager_llp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_manager_llp.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml), [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`cbu_identification`**
  * Defined in: [group_ownership.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/group_ownership.yaml)
  * Status: Declares Verb Palette
* **`company_secretary`**
  * Defined in: [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml)
  * Status: Declares Verb Palette
* **`compliance_officer`**
  * Defined in: [struct_uk_manager_llp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_manager_llp.yaml)
  * Status: Declares Verb Palette
* **`control_chain`**
  * Defined in: [group_ownership.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/group_ownership.yaml)
  * Status: Declares Verb Palette
* **`custodian`**
  * Defined in: [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`delegation`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`depositary`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml), [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml)
  * Status: Declares Verb Palette
* **`designated_member_1`**
  * Defined in: [struct_uk_manager_llp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_manager_llp.yaml)
  * Status: Declares Verb Palette
* **`designated_member_2`**
  * Defined in: [struct_uk_manager_llp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_manager_llp.yaml)
  * Status: Declares Verb Palette
* **`distributor`**
  * Defined in: [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml)
  * Status: Declares Verb Palette
* **`entity_research`**
  * Defined in: [group_ownership.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/group_ownership.yaml)
  * Status: Declares Verb Palette
* **`executing_broker`**
  * Defined in: [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml)
  * Status: Declares Verb Palette
* **`feeder`**
  * Defined in: [fund_administration.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/fund_administration.yaml)
  * Status: Declares Verb Palette
* **`fund`**
  * Defined in: [fund_administration.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/fund_administration.yaml)
  * Status: Declares Verb Palette
* **`general_partner`**
  * Defined in: [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`identifier`**
  * Defined in: [kyc_onboarding.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_onboarding.yaml)
  * Status: Declares Verb Palette
* **`ie_feeder`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml)
  * Status: Declares Verb Palette
* **`investment`**
  * Defined in: [fund_administration.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/fund_administration.yaml)
  * Status: Declares Verb Palette
* **`investment_adviser`**
  * Defined in: [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml)
  * Status: Declares Verb Palette
* **`investment_manager`**
  * Defined in: [fund_administration.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/fund_administration.yaml), [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml), [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml), [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml), [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`kyc_agreement`**
  * Defined in: [kyc_onboarding.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_onboarding.yaml)
  * Status: Declares Verb Palette
* **`legal_counsel`**
  * Defined in: [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml), [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`management_company`**
  * Defined in: [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml)
  * Status: Declares Verb Palette
* **`manco_group`**
  * Defined in: [fund_administration.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/fund_administration.yaml)
  * Status: Declares Verb Palette
* **`mandate`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml), [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml), [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml), [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`market_maker`**
  * Defined in: [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml)
  * Status: Declares Verb Palette
* **`mlro`**
  * Defined in: [struct_uk_manager_llp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_manager_llp.yaml)
  * Status: Declares Verb Palette
* **`ownership_chain`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_ie_ucits_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_ucits_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_lux_pe_scsp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_pe_scsp.yaml), [struct_lux_ucits_sicav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_ucits_sicav.yaml), [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml), [struct_uk_authorised_acs.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_acs.yaml), [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml), [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml), [struct_uk_manager_llp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_manager_llp.yaml), [struct_uk_private_equity_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_private_equity_lp.yaml), [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`participant`**
  * Defined in: [deal_lifecycle.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/deal_lifecycle.yaml)
  * Status: Declares Verb Palette
* **`partnership_capital`**
  * Defined in: [kyc_extended.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_extended.yaml)
  * Status: Declares Verb Palette
* **`prime_broker`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_aif_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_aif_icav.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml), [struct_lux_aif_raif.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_lux_aif_raif.yaml), [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`rate_card`**
  * Defined in: [deal_lifecycle.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/deal_lifecycle.yaml)
  * Status: Declares Verb Palette
* **`registrar`**
  * Defined in: [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml), [struct_uk_authorised_oeic.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_oeic.yaml)
  * Status: Declares Verb Palette
* **`regulatory`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`request`**
  * Defined in: [kyc_onboarding.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_onboarding.yaml)
  * Status: Declares Verb Palette
* **`resource_dictionary`**
  * Defined in: [product_service_taxonomy.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/product_service_taxonomy.yaml)
  * Status: Declares Verb Palette
* **`rule`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`rule_field`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`ruleset`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`secondary_prime_broker`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml), [struct_ie_hedge_icav.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_ie_hedge_icav.yaml)
  * Status: Declares Verb Palette
* **`service_resource_def_governance`**
  * Defined in: [registry_stewardship.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/registry_stewardship.yaml)
  * Status: Declares Verb Palette
* **`settlement_pattern`**
  * Defined in: [instrument_template.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/instrument_template.yaml)
  * Status: Declares Verb Palette
* **`sla`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`sub_adviser`**
  * Defined in: [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml)
  * Status: Declares Verb Palette
* **`tax_advisor`**
  * Defined in: [struct_us_private_fund_delaware_lp.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_private_fund_delaware_lp.yaml)
  * Status: Declares Verb Palette
* **`team`**
  * Defined in: [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
  * Status: Declares Verb Palette
* **`transfer_agent`**
  * Defined in: [struct_us_40act_closed_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_closed_end.yaml), [struct_us_40act_open_end.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_40act_open_end.yaml), [struct_us_etf_40act.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_us_etf_40act.yaml)
  * Status: Declares Verb Palette
* **`trust_control`**
  * Defined in: [kyc_extended.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_extended.yaml)
  * Status: Declares Verb Palette
* **`trustee`**
  * Defined in: [struct_uk_authorised_aut.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_aut.yaml)
  * Status: Declares Verb Palette
* **`ubo_discovery`**
  * Defined in: [group_ownership.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/group_ownership.yaml)
  * Status: Declares Verb Palette
* **`umbrella`**
  * Defined in: [fund_administration.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/fund_administration.yaml)
  * Status: Declares Verb Palette
* **`us_feeder`**
  * Defined in: [struct_hedge_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_hedge_cross_border.yaml)
  * Status: Declares Verb Palette
* **`us_parallel`**
  * Defined in: [struct_pe_cross_border.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_pe_cross_border.yaml)
  * Status: Declares Verb Palette
* **`valuation_agent`**
  * Defined in: [struct_uk_authorised_ltaf.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/struct_uk_authorised_ltaf.yaml)
  * Status: Declares Verb Palette

---

## ORPHAN-TOPO (Keys in GLOBAL_DAG but in NO Constellation Map)
These slots exist in the DAG transition topology but have no rich database table mapping or action interface (verb palette) defined on them. They represent lean topological states or structural constraints.

* **`application`**
* **`attribute`**
* **`billing_account_target`**
* **`billing_period`**
* **`book`**
* **`book_participant`**
* **`case_event`**
* **`cbu_corporate_action`**
* **`cbu_discovery_state`**
* **`cbu_disposition`**
* **`cbu_entity_relationship`**
* **`cbu_entity_role`**
* **`cbu_evidence`**
* **`clearance`**
* **`client_group_entity_review`**
* **`client_principal_relationship`**
* **`collateral_management`**
* **`corporate_action_event`**
* **`coverage`**
* **`deal_document`**
* **`deal_onboarding_request`**
* **`deal_participant`**
* **`deal_rate_card`**
* **`deal_sla`**
* **`deal_ubo_assessment`**
* **`doc_request`**
* **`edge`**
* **`entity_kyc`**
* **`entity_limited_company_ubo`**
* **`entity_proper_person`**
* **`holding`**
* **`investor`**
* **`investor_kyc`**
* **`investor_role`**
* **`kyc_decision`**
* **`kyc_service_agreement`**
* **`kyc_ubo_evidence`**
* **`kyc_ubo_registry`**
* **`manco`**
* **`mandate_outline`**
* **`outreach_request`**
* **`pricing_config`**
* **`product_service_condition`**
* **`proposal`**
* **`rate_card_line`**
* **`reconciliation`**
* **`red_flag`**
* **`remediation_event`**
* **`service_consumption`**
* **`service_version`**
* **`session_scope`**
* **`settlement_pattern_template`**
* **`shared_atom`**
* **`skeleton_build`**
* **`structure_template`**
* **`temporal`**
* **`trading_activity`**
* **`trading_profile_template`**
* **`ubo_evidence`**
* **`ubo_snapshot`**
* **`verb`**

---

## "WHAT I DID NOT DO" Ledger

In strict compliance with Phase 2 — Step 4 Prep (cont.) read-only constraints:
1. **No Source Edits**: Did not modify any code or configuration file in either repository.
2. **No Dependency Changes**: Did not touch Cargo.toml or Cargo.lock files.
3. **No Orphan Pruning**: Did not delete or align any of the orphan slots discovered in this check.
4. **No Git State Mutation (Except Report Commit)**: Staged and committed only this report file.
5. **No System Alterations**: Ran no service deployments or custom validations.
