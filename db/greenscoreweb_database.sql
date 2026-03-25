-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Hôte : 127.0.0.1:3306
-- Généré le : mar. 24 mars 2026 à 09:57
-- Version du serveur : 8.3.0
-- Version de PHP : 8.3.6

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Base de données : `greenscoreweb_database`
--

-- --------------------------------------------------------

--
-- Structure de la table `advice`
--

DROP TABLE IF EXISTS `advice`;
CREATE TABLE IF NOT EXISTS `advice` (
  `id` int NOT NULL AUTO_INCREMENT,
  `is_dev` tinyint(1) NOT NULL,
  `advice` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `title` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `icon` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=41 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Déchargement des données de la table `advice`
--

INSERT INTO `advice` (`id`, `is_dev`, `advice`, `title`, `icon`) VALUES
(1, 1, 'data.advice.optimize_sql', 'data.advice.optimize_sql_title', 'fa-solid fa-database'),
(2, 1, 'data.advice.minify_files', 'data.advice.minify_files_title', 'fa-brands fa-css'),
(3, 1, 'data.advice.server_cache', 'data.advice.server_cache_title', 'fa-solid fa-server'),
(4, 1, 'data.advice.reduce_http_requests', 'data.advice.reduce_http_requests_title', 'fa-solid fa-file'),
(5, 1, 'data.advice.use_webp', 'data.advice.use_webp_title', 'fa-solid fa-image'),
(6, 1, 'data.advice.lazy_loading', 'data.advice.lazy_loading_title', 'fa-solid fa-image'),
(7, 1, 'data.advice.avoid_loops', 'data.advice.avoid_loops_title', 'fa-solid fa-rotate-left'),
(8, 1, 'data.advice.eco_servers', 'data.advice.eco_servers_title', 'fa-solid fa-server'),
(9, 1, 'data.advice.use_cdn', 'data.advice.use_cdn_title', 'fa-solid fa-chart-simple'),
(10, 1, 'data.advice.reduce_resource_usage', 'data.advice.reduce_resource_usage_title', 'fa-solid fa-microchip'),
(11, 1, 'data.advice.efficient_algorithms', 'data.advice.efficient_algorithms_title', 'fa-solid fa-chart-diagram'),
(12, 1, 'data.advice.reduce_cookies', 'data.advice.reduce_cookies_title', 'fa-solid fa-cookie'),
(13, 1, 'data.advice.http_caching', 'data.advice.http_caching_title', 'fa-solid fa-server'),
(14, 1, 'data.advice.lightweight_frameworks', 'data.advice.lightweight_frameworks_title', 'fa-solid fa-book'),
(15, 1, 'data.advice.adapted_db', 'data.advice.adapted_db_title', 'fa-solid fa-database'),
(16, 1, 'data.advice.test_performance', 'data.advice.test_performance_title', 'fa-solid fa-code'),
(17, 1, 'data.advice.cloud_solutions', 'data.advice.cloud_solutions_title', 'fa-solid fa-cloud-arrow-up'),
(18, 1, 'data.advice.disable_logs', 'data.advice.disable_logs_title', 'fa-solid fa-file-lines'),
(19, 1, 'data.advice.offline_reports', 'data.advice.offline_reports_title', 'fa-solid fa-bug'),
(20, 1, 'data.advice.resource_audit', 'data.advice.resource_audit_title', 'fa-solid fa-network-wired'),
(21, 0, 'data.advice.adjust_brightness', 'data.advice.adjust_brightness_title', 'fa-solid fa-desktop'),
(22, 0, 'data.advice.close_unused_tabs', 'data.advice.close_unused_tabs_title', 'fa-solid fa-window-restore'),
(23, 0, 'data.advice.block_ads', 'data.advice.block_ads_title', 'fa-solid fa-bullhorn'),
(24, 0, 'data.advice.eco_search_engines', 'data.advice.eco_search_engines_title', 'fa-solid fa-leaf'),
(25, 0, 'data.advice.disable_autoplay', 'data.advice.disable_autoplay_title', 'fa-solid fa-photo-film'),
(26, 0, 'data.advice.download_necessary', 'data.advice.download_necessary_title', 'fa-solid fa-file'),
(27, 0, 'data.advice.clear_cache', 'data.advice.clear_cache_title', 'fa-solid fa-cookie'),
(28, 0, 'data.advice.limit_extensions', 'data.advice.limit_extensions_title', 'fa-solid fa-puzzle-piece'),
(29, 0, 'data.advice.data_saver_mode', 'data.advice.data_saver_mode_title', 'fa-solid fa-leaf'),
(30, 0, 'data.advice.mobile_optimized_sites', 'data.advice.mobile_optimized_sites_title', 'fa-solid fa-mobile'),
(31, 0, 'data.advice.low_res_video', 'data.advice.low_res_video_title', 'fa-solid fa-photo-film'),
(32, 0, 'data.advice.logout_unused', 'data.advice.logout_unused_title', 'fa-solid fa-right-from-bracket'),
(33, 0, 'data.advice.lightweight_browser', 'data.advice.lightweight_browser_title', 'fa-solid fa-window-maximize'),
(34, 0, 'data.advice.wifi_over_data', 'data.advice.wifi_over_data_title', 'fa-solid fa-wifi'),
(35, 0, 'data.advice.avoid_heavy_ads', 'data.advice.avoid_heavy_ads_title', 'fa-solid fa-bullhorn'),
(36, 0, 'data.advice.plan_searches', 'data.advice.plan_searches_title', 'fa-solid fa-globe'),
(37, 0, 'data.advice.text_mode', 'data.advice.text_mode_title', 'fa-solid fa-text-height'),
(38, 0, 'data.advice.share_links', 'data.advice.share_links_title', 'fa-solid fa-link'),
(39, 0, 'data.advice.disable_push', 'data.advice.disable_push_title', 'fa-solid fa-bell'),
(40, 0, 'data.advice.close_background_apps', 'data.advice.close_background_apps_title', 'fa-solid fa-square-xmark');

-- --------------------------------------------------------

--
-- Structure de la table `equivalent`
--

DROP TABLE IF EXISTS `equivalent`;
CREATE TABLE IF NOT EXISTS `equivalent` (
  `id` int NOT NULL AUTO_INCREMENT,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `equivalent` double NOT NULL,
  `icon_thumbnail` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=130 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Déchargement des données de la table `equivalent`
--

INSERT INTO `equivalent` (`id`, `name`, `equivalent`, `icon_thumbnail`) VALUES
(1, 'data.equivalent.lille_marseille', 0.004, 'car.png'),
(2, 'data.equivalent.paris_londres', 0.73, 'train-paris-londres.png'),
(3, 'data.equivalent.emails', 2000, 'email.png'),
(4, 'data.equivalent.search', 5000, 'search.png'),
(5, 'data.equivalent.streaming', 17.86, 'youtube.png'),
(6, 'data.equivalent.bus', 36, 'bus.png'),
(7, 'data.equivalent.motorbike', 4.54, 'motorbike.png'),
(8, 'data.equivalent.car', 4.54, 'car.png'),
(9, 'data.equivalent.tap_water', 7692, 'faucet.png'),
(10, 'data.equivalent.paris_ny', 0.0007, 'new-york.png');

-- --------------------------------------------------------

--
-- Structure de la table `monitored_website`
--

DROP TABLE IF EXISTS `monitored_website`;
CREATE TABLE IF NOT EXISTS `monitored_website` (
  `id` int NOT NULL AUTO_INCREMENT,
  `user_id` int DEFAULT NULL,
  `url_domain` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `url_full` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `queries_quantity` int DEFAULT NULL,
  `carbon_footprint` double DEFAULT NULL,
  `data_transferred` double DEFAULT NULL,
  `resources` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci,
  `loading_time` double DEFAULT NULL,
  `country` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `creation_date` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `IDX_7458B0D5A76ED395` (`user_id`)
) ENGINE=InnoDB AUTO_INCREMENT=5014 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Déchargement des données de la table `monitored_website`
--

INSERT INTO `monitored_website` (`id`, `user_id`, `url_domain`, `url_full`, `queries_quantity`, `carbon_footprint`, `data_transferred`, `resources`, `loading_time`, `country`, `creation_date`) VALUES
(1, 1005, 'youtube.com', 'https://www.youtube.com/', 17, 1.61, 1979313, '2324467', 1.978, 'France', '2025-01-21 17:11:08'),
(2, 1005, 'amazon.com', 'https://www.amazon.com/', 288, 19.19, 136183, '2492594', 1.741, 'France', '2025-01-21 17:11:16'),
(3, 1005, 'amazon.com', 'https://www.amazon.com/b/?_encoding=UTF8&node=23634165011&pd_rd_w=GpurF&content-id=amzn1.sym.9e3167b9-3458-4bd3-a9ee-02b4fc17a8ef&pf_rd_p=9e3167b9-3458-4bd3-a9ee-02b4fc17a8ef&pf_rd_r=2C7HAPA6MT0BNGQK0QFR&pd_rd_wg=3xyBx&pd_rd_r=fbd3236c-1a5c-4bb4-a548-f60496b6af44&ref_=pd_hp_d_hero_unk', 104, 1.06, 113392, '2423335', 1.392, 'France', '2025-01-21 17:11:19'),
(4, 1005, 'amazon.com', 'https://www.amazon.com/b/ref=s9_acss_bw_cg_HOL21GFG_1a1_w?node=23634102011&pf_rd_p=9b4a235a-d43f-40cc-972b-378d04bad40f&pf_rd_r=0Y97H5FV8MB6V5K32RPH', 68, 0.72, 908784, '1086969', 1.297, 'France', '2025-01-21 17:11:22'),
(5, 1005, 'amazon.fr', 'https://www.amazon.fr/?tag=admarketpla00-21&ref=pd_sl_780d6e6918d3edbf67fd91f28348c5e4bb827a5c095523bc0558ce2d&mfadid=adm', 5, 0.33, 0, '0', -1737475884.256, 'France', '2025-01-21 17:11:25'),
(6, 1005, 'amazon.com', 'https://www.amazon.com/ref=nav_logo', 89, 0.9, 141451, '502869', 1.216, 'France', '2025-01-21 17:11:27'),
(155, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee', 1, 0.07, 0, '0', -1768573291.455, 'France', '2026-01-16 14:21:31'),
(156, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/login', 36, 0.38, 226897, '2062071', 0.509, 'France', '2026-01-16 14:21:32'),
(157, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/', 60, 0.63, 331124, '4596208', 20.179, 'France', '2026-01-16 14:21:51'),
(158, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee', 1, 0.07, 0, '0', -1768573323.088, 'France', '2026-01-16 14:22:03'),
(159, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=4.21&pageSize=5095497&loadingTime=2214.046&queriesQuantity=421', 28, 1.87, 14695, '220195', 0.194, 'France', '2026-01-16 14:24:01'),
(160, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=4.21&pageSize=5095497&loadingTime=2214.046&queriesQuantity=421', 32, 0.33, 14695, '1288570', 0.259, 'France', '2026-01-16 14:24:06'),
(161, 1005, 'amazon.fr', 'https://www.amazon.fr/', 432, 4.32, 839981, '5095497', 2336.204, 'France', '2026-01-16 14:25:43'),
(162, 1005, 'amazon.fr', 'https://www.amazon.fr/b/?_encoding=UTF8&node=215455023031&pd_rd_w=j4qgf&content-id=amzn1.sym.3707a5d3-a1a2-4f28-bad1-03a047718aba&pf_rd_p=3707a5d3-a1a2-4f28-bad1-03a047718aba&pf_rd_r=1QJ9WG990Q2W7CYK179W&pd_rd_wg=0KY6z&pd_rd_r=a741bddc-9d59-49f8-a0a7-a132191de725&ref_=pd_hp_d_hero_unk', 0, 0, 0, '0', 0, 'France', '2026-01-16 14:25:43'),
(163, 1005, 'amazon.fr', 'https://www.amazon.fr/b/?_encoding=UTF8&node=215455023031&pd_rd_w=j4qgf&content-id=amzn1.sym.3707a5d3-a1a2-4f28-bad1-03a047718aba&pf_rd_p=3707a5d3-a1a2-4f28-bad1-03a047718aba&pf_rd_r=1QJ9WG990Q2W7CYK179W&pd_rd_wg=0KY6z&pd_rd_r=a741bddc-9d59-49f8-a0a7-a132191de725&ref_=pd_hp_d_hero_unk', 1, 0.01, 0, '0', -1768573543.769, 'France', '2026-01-16 14:25:43'),
(164, 1005, 'amazon.fr', 'https://www.amazon.fr/', 18, 0.19, 88040, '349941', 0.164, 'France', '2026-01-16 14:25:44'),
(165, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.31&pageSize=5274328&loadingTime=4.872&queriesQuantity=350', 28, 1.87, 14699, '220199', 0.412, 'France', '2026-01-16 14:25:52'),
(166, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.84&pageSize=5503404&loadingTime=19.324&queriesQuantity=376', 1, 0.07, 0, '0', -1768573568.224, 'France', '2026-01-16 14:26:08'),
(167, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.84&pageSize=5503404&loadingTime=19.324&queriesQuantity=376', 32, 0.33, 14732, '1288608', 0.254, 'France', '2026-01-16 14:26:09'),
(168, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.31&pageSize=5274328&loadingTime=4.872&queriesQuantity=350', 32, 0.33, 14699, '1288574', 0.516, 'France', '2026-01-16 14:26:10'),
(169, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee', 65, 0.69, 331124, '5588714', 310.773, 'France', '2026-01-16 14:26:42'),
(170, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/mon-organisation', 70, 0.74, 542127, '5806696', 332.247, 'France', '2026-01-16 14:27:04'),
(171, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/mes-donnees', 70, 0.74, 542127, '5814300', 332.713, 'France', '2026-01-16 14:27:04'),
(172, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/', 71, 0.75, 548912, '5820908', 346.133, 'France', '2026-01-16 14:27:19'),
(173, 1005, 'google.com', 'https://www.google.com/search?client=firefox-b-d&q=temu', 7, 0.47, 0, '52006', 0.116, 'France', '2026-01-16 14:31:10'),
(174, 1005, 'google.com', 'https://www.google.com/search?client=firefox-b-d&q=temu', 80, 0.84, 902885, '2427760', 1.711, 'France', '2026-01-16 14:31:12'),
(175, 1005, 'google.com', 'https://www.google.com/aclk?sa=L&ai=DChsSEwib8NSKo5CSAxWiREECHbCDEW0YACICCAEQABoCd3M&co=1&gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&sig=AOD64_2tb0bR_B9PBMlBh7JCeb-0cgAOtQ&q&adurl&ved=2ahUKEwiivtCKo5CSAxVtU6QEHSckCpQQ0Qx6BAgKEAE&nis=8&ch=1', 4, 0.04, 0, '649', 0.133, 'France', '2026-01-16 14:31:12'),
(176, 1005, 'google.com', 'https://www.google.com/aclk?sa=L&ai=DChsSEwib8NSKo5CSAxWiREECHbCDEW0YACICCAEQABoCd3M&co=1&gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&sig=AOD64_2tb0bR_B9PBMlBh7JCeb-0cgAOtQ&q&adurl&ved=2ahUKEwiivtCKo5CSAxVtU6QEHSckCpQQ0Qx6BAgKEAE&nis=8&ch=1', 13, 0.13, 0, '1364', 0.314, 'France', '2026-01-16 14:31:13'),
(177, 1005, 'temu.com', 'https://www.temu.com/ul/kuiper/un2.html?_p_rfs=1&subj=un-search-web&_p_jump_id=960&_x_vst_scene=adg&locale_override=69~~EUR&search_key=Temu&_p_rfs=1&_x_ads_channel=google&_x_ads_sub_channel=search&_x_ads_account=9449646839&_x_ads_set=20022547341&_x_ads_id=148424958636&_x_ads_creative_id=656111305084&_x_ns_source=g&_x_ns_gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&_x_ns_placement=&_x_ns_match_type=e&_x_ns_ad_position=&_x_ns_product_id=&_x_ns_target=&_x_ns_devicemodel=&_x_ns_wbraid=Cj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA&_x_ns_gbraid=0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg&_x_ns_keyword=temu&_x_ns_targetid=kwd-4583699489&gad_source=1&gad_campaignid=20022547341&gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE', 0, 0, 0, '0', 1768573873.137, 'France', '2026-01-16 14:31:13'),
(178, 1005, 'temu.com', 'https://www.temu.com/ul/kuiper/un2.html?_p_rfs=1&subj=un-search-web&_p_jump_id=960&_x_vst_scene=adg&search_key=Temu&_x_ads_channel=google&_x_ads_sub_channel=search&_x_ads_account=9449646839&_x_ads_set=20022547341&_x_ads_id=148424958636&_x_ads_creative_id=656111305084&_x_ns_source=g&_x_ns_gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&_x_ns_placement=&_x_ns_match_type=e&_x_ns_ad_position=&_x_ns_product_id=&_x_ns_target=&_x_ns_devicemodel=&_x_ns_wbraid=Cj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA&_x_ns_gbraid=0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg&_x_ns_keyword=temu&_x_ns_targetid=kwd-4583699489&gad_source=1&gad_campaignid=20022547341&gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&adg_ctx=f-cb2d7e57', 120, 1.25, 794866, '4270700', 0.775, 'France', '2026-01-16 14:31:14'),
(179, 1005, 'temu.com', 'https://www.temu.com/ul/kuiper/un2.html?_p_rfs=1&subj=un-search-web&_p_jump_id=960&_x_vst_scene=adg&locale_override=69~~EUR&search_key=Temu&_p_rfs=1&_x_ads_channel=google&_x_ads_sub_channel=search&_x_ads_account=9449646839&_x_ads_set=20022547341&_x_ads_id=148424958636&_x_ads_creative_id=656111305084&_x_ns_source=g&_x_ns_gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&_x_ns_placement=&_x_ns_match_type=e&_x_ns_ad_position=&_x_ns_product_id=&_x_ns_target=&_x_ns_devicemodel=&_x_ns_wbraid=Cj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA&_x_ns_gbraid=0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg&_x_ns_keyword=temu&_x_ns_targetid=kwd-4583699489&gad_source=1&gad_campaignid=20022547341&gclid=EAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE', 149, 1.55, 956821, '5080475', 1.08, 'France', '2026-01-16 14:31:15'),
(180, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.temu.com%2Ful%2Fkuiper%2Fun2.html%3F_p_rfs%3D1%26subj%3Dun-search-web%26_p_jump_id%3D960%26_x_vst_scene%3Dadg%26locale_override%3D69%7E%7EEUR%26search_key%3DTemu%26_p_rfs%3D1%26_x_ads_channel%3Dgoogle%26_x_ads_sub_channel%3Dsearch%26_x_ads_account%3D9449646839%26_x_ads_set%3D20022547341%26_x_ads_id%3D148424958636%26_x_ads_creative_id%3D656111305084%26_x_ns_source%3Dg%26_x_ns_gclid%3DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE%26_x_ns_placement%3D%26_x_ns_match_type%3De%26_x_ns_ad_position%3D%26_x_ns_product_id%3D%26_x_ns_target%3D%26_x_ns_devicemodel%3D%26_x_ns_wbraid%3DCj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA%26_x_ns_gbraid%3D0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg%26_x_ns_keyword%3Dtemu%26_x_ns_targetid%3Dkwd-4583699489%26gad_source%3D1%26gad_campaignid%3D20022547341%26gclid%3DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&totalConsu=1.25&pageSize=5641536&loadingTime=6.473&queriesQuantity=300', 1, 0.07, 0, '0', -1768573880.75, 'France', '2026-01-16 14:31:20'),
(181, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.temu.com%2Ful%2Fkuiper%2Fun2.html%3F_p_rfs%3D1%26subj%3Dun-search-web%26_p_jump_id%3D960%26_x_vst_scene%3Dadg%26locale_override%3D69%7E%7EEUR%26search_key%3DTemu%26_p_rfs%3D1%26_x_ads_channel%3Dgoogle%26_x_ads_sub_channel%3Dsearch%26_x_ads_account%3D9449646839%26_x_ads_set%3D20022547341%26_x_ads_id%3D148424958636%26_x_ads_creative_id%3D656111305084%26_x_ns_source%3Dg%26_x_ns_gclid%3DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE%26_x_ns_placement%3D%26_x_ns_match_type%3De%26_x_ns_ad_position%3D%26_x_ns_product_id%3D%26_x_ns_target%3D%26_x_ns_devicemodel%3D%26_x_ns_wbraid%3DCj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA%26_x_ns_gbraid%3D0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg%26_x_ns_keyword%3Dtemu%26_x_ns_targetid%3Dkwd-4583699489%26gad_source%3D1%26gad_campaignid%3D20022547341%26gclid%3DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&totalConsu=1.25&pageSize=5641536&loadingTime=6.473&queriesQuantity=300', 32, 0.33, 14680, '1288556', 0.268, 'France', '2026-01-16 14:31:51'),
(182, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fgreenscore.alwaysdata.net%2Fderniere-page-consultee%3Fcountry%3DFrance%26url_full%3Dhttps%253A%252F%252Fwww.temu.com%252Ful%252Fkuiper%252Fun2.html%253F_p_rfs%253D1%2526subj%253Dun-search-web%2526_p_jump_id%253D960%2526_x_vst_scene%253Dadg%2526locale_override%253D69%257E%257EEUR%2526search_key%253DTemu%2526_p_rfs%253D1%2526_x_ads_channel%253Dgoogle%2526_x_ads_sub_channel%253Dsearch%2526_x_ads_account%253D9449646839%2526_x_ads_set%253D20022547341%2526_x_ads_id%253D148424958636%2526_x_ads_creative_id%253D656111305084%2526_x_ns_source%253Dg%2526_x_ns_gclid%253DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE%2526_x_ns_placement%253D%2526_x_ns_match_type%253De%2526_x_ns_ad_position%253D%2526_x_ns_product_id%253D%2526_x_ns_target%253D%2526_x_ns_devicemodel%253D%2526_x_ns_wbraid%253DCj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA%2526_x_ns_gbraid%253D0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg%2526_x_ns_keyword%253Dtemu%2526_x_ns_targetid%253Dkwd-4583699489%2526gad_source%253D1%2526gad_campaignid%253D20022547341%2526gclid%253DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE%26totalConsu%3D1.25%26pageSize%3D5641536%26loadingTime%3D6.473%26queriesQuantity%3D300&totalConsu=0.33&pageSize=1288556&loadingTime=0.268&queriesQuantity=32', 1, 0.07, 0, '0', -1768573911.111, 'France', '2026-01-16 14:31:51'),
(183, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fgreenscore.alwaysdata.net%2Fderniere-page-consultee%3Fcountry%3DFrance%26url_full%3Dhttps%253A%252F%252Fwww.temu.com%252Ful%252Fkuiper%252Fun2.html%253F_p_rfs%253D1%2526subj%253Dun-search-web%2526_p_jump_id%253D960%2526_x_vst_scene%253Dadg%2526locale_override%253D69%257E%257EEUR%2526search_key%253DTemu%2526_p_rfs%253D1%2526_x_ads_channel%253Dgoogle%2526_x_ads_sub_channel%253Dsearch%2526_x_ads_account%253D9449646839%2526_x_ads_set%253D20022547341%2526_x_ads_id%253D148424958636%2526_x_ads_creative_id%253D656111305084%2526_x_ns_source%253Dg%2526_x_ns_gclid%253DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE%2526_x_ns_placement%253D%2526_x_ns_match_type%253De%2526_x_ns_ad_position%253D%2526_x_ns_product_id%253D%2526_x_ns_target%253D%2526_x_ns_devicemodel%253D%2526_x_ns_wbraid%253DCj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA%2526_x_ns_gbraid%253D0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg%2526_x_ns_keyword%253Dtemu%2526_x_ns_targetid%253Dkwd-4583699489%2526gad_source%253D1%2526gad_campaignid%253D20022547341%2526gclid%253DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE%26totalConsu%3D1.25%26pageSize%3D5641536%26loadingTime%3D6.473%26queriesQuantity%3D300&totalConsu=0.33&pageSize=1288556&loadingTime=0.268&queriesQuantity=32', 32, 0.33, 14726, '1288602', 0.371, 'France', '2026-01-16 14:32:00'),
(184, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.temu.com%2Ful%2Fkuiper%2Fun2.html%3F_p_rfs%3D1%26subj%3Dun-search-web%26_p_jump_id%3D960%26_x_vst_scene%3Dadg%26locale_override%3D69%7E%7EEUR%26search_key%3DTemu%26_p_rfs%3D1%26_x_ads_channel%3Dgoogle%26_x_ads_sub_channel%3Dsearch%26_x_ads_account%3D9449646839%26_x_ads_set%3D20022547341%26_x_ads_id%3D148424958636%26_x_ads_creative_id%3D656111305084%26_x_ns_source%3Dg%26_x_ns_gclid%3DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE%26_x_ns_placement%3D%26_x_ns_match_type%3De%26_x_ns_ad_position%3D%26_x_ns_product_id%3D%26_x_ns_target%3D%26_x_ns_devicemodel%3D%26_x_ns_wbraid%3DCj4KCAiAvaLLBhAcEi4AdKTQ-B-mn_cwR8S_uWwNtG6BWz8cO6PKtEJdeUtsDnZhjlb1URhDHXB2ZsgjGgJinA%26_x_ns_gbraid%3D0AAAAAo4mICFJyJt4DR1XQfiUUQCAcfyjg%26_x_ns_keyword%3Dtemu%26_x_ns_targetid%3Dkwd-4583699489%26gad_source%3D1%26gad_campaignid%3D20022547341%26gclid%3DEAIaIQobChMIm_DUiqOQkgMVokRBAh2wgxFtEAAYASAAEgLD7fD_BwE&totalConsu=1.25&pageSize=5641536&loadingTime=6.473&queriesQuantity=300', 32, 0.33, 14680, '1288556', 0.268, 'France', '2026-01-16 14:32:00'),
(5000, 1005, 'temu.com', 'https://www.temu.com/?_x_ns_irclickid=wBszsYSi4xycUpIVAIW582iYUkpSn3UC2WHGR40&_x_ads_account=18350&_x_ads_id=1580294&_x_ns_iradname=Online%20Tracking%20Link&_x_ns_iradsize=&_x_ns_prodsku=&_x_ns_irmptype=mediapartner&_x_ns_sharedid=firefox&_x_ns_ts=1768913287842&_x_ns_randint=6684050&_x_ns_adtype=ONLINE_TRACKING_LINK&_p_rfs=1&irgwc=1&afsrc=1&_x_ns_irmpgroupname=%22cx%22&_x_ads_channel=impact&_x_ns_mp_value2=&_x_ns_mp_value3=&_x_ns_irmpname=Firefox%20Browser%20-%20adMarketplace&_x_ns_irpid=2626476&_bg_fs=1&_p_jump_id=866&_x_vst_scene=adg', 38, 0.32, 35338, '969926', 0.133, 'France', '2026-01-20 12:48:09'),
(5001, 1005, 'temu.com', 'https://www.temu.com/?_x_ns_irclickid=wBszsYSi4xycUpIVAIW582iYUkpSn3UC2WHGR40&_x_ads_account=18350&_x_ads_id=1580294&_x_ns_iradname=Online%20Tracking%20Link&_x_ns_iradsize=&_x_ns_prodsku=&_x_ns_irmptype=mediapartner&_x_ns_sharedid=firefox&_x_ns_ts=1768913287842&_x_ns_randint=6684050&_x_ns_adtype=ONLINE_TRACKING_LINK&_p_rfs=1&irgwc=1&afsrc=1&_x_ns_irmpgroupname=%22cx%22&_x_ads_channel=impact&_x_ns_mp_value2=&_x_ns_mp_value3=&_x_ns_irmpname=Firefox%20Browser%20-%20adMarketplace&_x_ns_irpid=2626476&_bg_fs=1&_p_jump_id=866&_x_vst_scene=adg', 398, 3.36, 222806, '1947908', 9.687, 'France', '2026-01-20 12:48:24'),
(5002, 1005, 'temu.com', 'https://www.temu.com/?_x_ns_irclickid=wBszsYSi4xycUpIVAIW582iYUkpSn3UC2WHGR40&_x_ads_account=18350&_x_ads_id=1580294&_x_ns_iradname=Online%20Tracking%20Link&_x_ns_iradsize=&_x_ns_prodsku=&_x_ns_irmptype=mediapartner&_x_ns_sharedid=firefox&_x_ns_ts=1768913287842&_x_ns_randint=6684050&_x_ns_adtype=ONLINE_TRACKING_LINK&_p_rfs=1&irgwc=1&afsrc=1&_x_ns_irmpgroupname=%22cx%22&_x_ads_channel=impact&_x_ns_mp_value2=&_x_ns_mp_value3=&_x_ns_irmpname=Firefox%20Browser%20-%20adMarketplace&_x_ns_irpid=2626476&_bg_fs=1&_p_jump_id=866&_x_vst_scene=adg', 454, 3.83, 230480, '2013976', 68.082, 'France', '2026-01-20 12:56:53'),
(5008, 1005, NULL, 'http://test.com', 5, 0.5, 10, NULL, 1.2, 'FR', '2026-01-28 14:17:02'),
(5009, 1005, NULL, 'http://test.com', 5, 0.5, 10, NULL, 1.2, 'FR', '2026-01-28 14:25:55'),
(5010, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.31&pageSize=5274328&loadingTime=4.872&queriesQuantity=350', 28, 1.87, 14699, '220199', 0.412, 'France', '2026-01-16 14:25:52'),
(5011, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.84&pageSize=5503404&loadingTime=19.324&queriesQuantity=376', 1, 0.07, 0, '0', -1768573568.224, 'France', '2026-01-16 14:26:08'),
(5012, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.84&pageSize=5503404&loadingTime=19.324&queriesQuantity=376', 32, 0.33, 14732, '1288608', 0.254, 'France', '2026-01-16 14:26:09'),
(5013, 1005, 'greenscore.alwaysdata.net', 'https://greenscore.alwaysdata.net/derniere-page-consultee?country=France&url_full=https%3A%2F%2Fwww.amazon.fr%2F&totalConsu=3.31&pageSize=5274328&loadingTime=4.872&queriesQuantity=350', 32, 0.33, 14699, '1288574', 0.516, 'France', '2026-01-16 14:26:10');

-- --------------------------------------------------------

--
-- Structure de la table `organisation`
--

DROP TABLE IF EXISTS `organisation`;
CREATE TABLE IF NOT EXISTS `organisation` (
  `id` int NOT NULL AUTO_INCREMENT,
  `organisation_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `organisation_code` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `siret` varchar(14) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=159 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Déchargement des données de la table `organisation`
--

INSERT INTO `organisation` (`id`, `organisation_name`, `organisation_code`, `siret`) VALUES
(149, 'grios', 'VLDKW9Z0', 'null'),
(150, 'fejzfhjefezj', '08RA5GEZ', '\"121202121002\"'),
(151, 'Salut à tous', 'RXRD43DO', '\"7845485784578'),
(152, 'fbehfehf', 'AUMFJWXO', '\"fehgfyehf\"'),
(153, 'Organisation', 'SZB1YHJT', '\"1224510152210'),
(154, 'fvgfvevfhgfvergfver', 'T0Y8MRXS', 'null'),
(155, 'dadadada', 'Y4RJIO3C', '\"1111111111111'),
(156, 'gjdfgjdfg', 'SXCE6K7I', '11111111111111'),
(157, 'dzadazdza', 'YISPWGH4', '11111111111111'),
(158, 'mmmmmmmmmm', 'ZD2W0NHM', '22222222222222');

-- --------------------------------------------------------

--
-- Structure de la table `organisation_user`
--

DROP TABLE IF EXISTS `organisation_user`;
CREATE TABLE IF NOT EXISTS `organisation_user` (
  `organisation_id` int NOT NULL,
  `user_id` int NOT NULL,
  `est_admin` tinyint(1) DEFAULT NULL,
  KEY `organisation_id` (`organisation_id`),
  KEY `user_id` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Structure de la table `service`
--

DROP TABLE IF EXISTS `service`;
CREATE TABLE IF NOT EXISTS `service` (
  `id` int NOT NULL AUTO_INCREMENT,
  `nom` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `id_organisation` int DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `id_organisation` (`id_organisation`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Déchargement des données de la table `service`
--

INSERT INTO `service` (`id`, `nom`, `id_organisation`) VALUES
(1, 'Test1', 158),
(2, 'Test2', 158);

-- --------------------------------------------------------

--
-- Structure de la table `sessions`
--

DROP TABLE IF EXISTS `sessions`;
CREATE TABLE IF NOT EXISTS `sessions` (
  `id` varchar(128) NOT NULL,
  `data` longblob NOT NULL,
  `expires_at` bigint DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Déchargement des données de la table `sessions`
--

INSERT INTO `sessions` (`id`, `data`, `expires_at`) VALUES
('MF_PukaA6ejJYoxA6s2h1w', 0x7b226964223a2d35333635383231313537393632313930393835303736343637343433353335323932383436342c2264617461223a7b22757365725f66756c6c223a7b226f7267616e69736174696f6e223a7b22636f6465223a225a443257304e484d222c226964223a3135382c226e6f6d223a226d6d6d6d6d6d6d6d6d6d222c227369726574223a223232323232323232323232323232227d2c2273657276696365223a7b226964223a322c2269645f6f7267616e69736174696f6e223a3135382c226e6f6d223a225465737432227d2c2275736572223a7b22656d61696c223a22726f62696e636279363440676d61696c2e636f6d222c226573745f61646d696e223a747275652c226964223a313030352c2269645f6f7267616e69736174696f6e223a3135382c2269645f73657276696365223a6e756c6c2c226e6f6d223a22436f6e6368657a2d426f756579746f75222c227072656e6f6d223a22526f62696e222c22746f74616c5f636172626f6e5f666f6f747072696e74223a302e307d7d7d2c226578706972795f64617465223a5b323032362c38322c31332c31352c302c3434313835313830302c302c302c305d7d, 1774271700);

-- --------------------------------------------------------

--
-- Structure de la table `user`
--

DROP TABLE IF EXISTS `user`;
CREATE TABLE IF NOT EXISTS `user` (
  `id` int NOT NULL AUTO_INCREMENT,
  `service_id` int DEFAULT NULL,
  `email` varchar(180) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `first_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `last_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  `total_carbon_footprint` double NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`),
  KEY `id_service` (`service_id`)
) ENGINE=InnoDB AUTO_INCREMENT=1007 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Déchargement des données de la table `user`
--

INSERT INTO `user` (`id`, `service_id`, `email`, `password`, `first_name`, `last_name`, `total_carbon_footprint`) VALUES
(1004, NULL, 'theo.mirambeaux@gmail.com', '$2b$12$pEQM8JeVxPFzmFiHq614Oen5CP.Tv6H/1mdZhVqo0zL1qJ/UKaoPy', 'Theo', 'Mirambeau', 0),
(1005, NULL, 'robincby64@gmail.com', '$2b$12$nx6QTShZUArINKKNmWXmL.uq/jGokvvnq9bMK0HKsXnnBRcvf7Xqa', 'Robin', 'Conchez-Boueytou', 0),
(1006, NULL, 'theo@gmail.com', '$2b$12$iydaySwtHDde9.51FMjmUeROOkBKcSXMXOsaW99yRGhG.ezmJ5CTe', 'theo', 'LaGrossealamereTheo', 0);

-- --------------------------------------------------------

--
-- Structure de la table `user_equivalent`
--

DROP TABLE IF EXISTS `user_equivalent`;
CREATE TABLE IF NOT EXISTS `user_equivalent` (
  `user_id` int NOT NULL,
  `equivalent_id` int NOT NULL,
  PRIMARY KEY (`user_id`,`equivalent_id`),
  KEY `IDX_5D8A3F6FA76ED395` (`user_id`),
  KEY `IDX_5D8A3F6F7C455263` (`equivalent_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Déchargement des données de la table `user_equivalent`
--

INSERT INTO `user_equivalent` (`user_id`, `equivalent_id`) VALUES
(1005, 3),
(1005, 4),
(1005, 9);

-- --------------------------------------------------------

--
-- Structure de la table `_sqlx_test_databases`
--

DROP TABLE IF EXISTS `_sqlx_test_databases`;
CREATE TABLE IF NOT EXISTS `_sqlx_test_databases` (
  `db_name` text COLLATE utf8mb4_general_ci NOT NULL,
  `test_path` text COLLATE utf8mb4_general_ci NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`db_name`(63))
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Déchargement des données de la table `_sqlx_test_databases`
--

INSERT INTO `_sqlx_test_databases` (`db_name`, `test_path`, `created_at`) VALUES
('_sqlx_test_S_M4NtxLUiXLAhoWIhPGzhbtpMMRdIo_SasICAMum9ZZ_gmLt3Aq', 'greenscore_test::test_green_score_my_data_dynamic', '2026-03-17 08:22:41'),
('_sqlx_test_k8Tm94YI47pjyqvshNMyC6KnLR1VOqxuktN_Pxl2HphiFP5V0GGq', 'greenscore_test::devrait_retourner_A_si_my_data_et_empreinte_proche_du_meilleur_utilisateur', '2026-03-17 08:27:18'),
('_sqlx_test_yYJ4QwDEGZf3HcPihqAe_xFCPaeNOtJP2LhFIZ8O4pLXpYp987Cm', 'greenscore_test::devrait_retourner_NA_si_my_data_et_aucune_donnee_en_base', '2026-03-17 08:26:44');

--
-- Contraintes pour les tables déchargées
--

--
-- Contraintes pour la table `monitored_website`
--
ALTER TABLE `monitored_website`
  ADD CONSTRAINT `FK_7458B0D5A76ED395` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE SET NULL;

--
-- Contraintes pour la table `organisation_user`
--
ALTER TABLE `organisation_user`
  ADD CONSTRAINT `organisation_user_ibfk_1` FOREIGN KEY (`organisation_id`) REFERENCES `organisation` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  ADD CONSTRAINT `organisation_user_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Contraintes pour la table `service`
--
ALTER TABLE `service`
  ADD CONSTRAINT `service_ibfk_1` FOREIGN KEY (`id_organisation`) REFERENCES `organisation` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Contraintes pour la table `user`
--
ALTER TABLE `user`
  ADD CONSTRAINT `user_ibfk_1` FOREIGN KEY (`service_id`) REFERENCES `service` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Contraintes pour la table `user_equivalent`
--
ALTER TABLE `user_equivalent`
  ADD CONSTRAINT `FK_5D8A3F6F7C455263` FOREIGN KEY (`equivalent_id`) REFERENCES `equivalent` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `FK_5D8A3F6FA76ED395` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
