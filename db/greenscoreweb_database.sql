-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Hôte : 127.0.0.1:3306
-- Généré le : lun. 12 jan. 2026 à 20:34
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
`advice` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
`title` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
`icon` varchar(100) COLLATE utf8mb4_unicode_ci NOT NULL,
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
`name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
`equivalent` double NOT NULL,
`icon_thumbnail` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=130 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

--
-- Déchargement des données de la table `equivalent`
--

INSERT INTO `equivalent` (`id`, `name`, `equivalent`, `icon_thumbnail`) VALUES
(1, 'data.equivalent.lille_nimes', 0.004, 'car.png'),
(2, 'data.equivalent.paris_berlin', 0.163, 'train-paris-berlin.png'),
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
`url_domain` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
`url_full` longtext COLLATE utf8mb4_unicode_ci,
`queries_quantity` int DEFAULT NULL,
`carbon_footprint` double DEFAULT NULL,
`data_transferred` double DEFAULT NULL,
`resources` longtext COLLATE utf8mb4_unicode_ci,
`loading_time` double DEFAULT NULL,
`country` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
`creation_date` datetime DEFAULT CURRENT_TIMESTAMP,
PRIMARY KEY (`id`),
KEY `IDX_7458B0D5A76ED395` (`user_id`)
) ENGINE=InnoDB AUTO_INCREMENT=153 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


-- --------------------------------------------------------

--
-- Structure de la table `organisation`
--

DROP TABLE IF EXISTS `organisation`;
CREATE TABLE IF NOT EXISTS `organisation` (
`id` int NOT NULL AUTO_INCREMENT,
`organisation_name` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
`organisation_code` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL,
`city` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
`siret` varchar(14) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=126 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


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
('tNQ_fYInLH2q_kZazJA4Vw', 0x7b226964223a3131353933363534313131343833333138383335373634313638383736393036313936373032382c2264617461223a7b226163636f756e74223a7b22636f6465223a224a36345834555056222c226964223a3237382c226e6f6d223a2274657374222c22726f6c65223a226f7267616e69736174696f6e222c227369726574223a6e756c6c7d7d2c226578706972795f64617465223a5b323032362c31322c32312c33342c342c383039363630302c302c302c305d7d, 1768253644);

-- --------------------------------------------------------

--
-- Structure de la table `user`
--

DROP TABLE IF EXISTS `user`;
CREATE TABLE IF NOT EXISTS `user` (
`id` int NOT NULL AUTO_INCREMENT,
`organisation_id` int DEFAULT NULL,
`email` varchar(180) COLLATE utf8mb4_unicode_ci NOT NULL,
`roles` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT '(DC2Type:json)',
`password` varchar(255) COLLATE utf8mb4_unicode_ci NOT NULL,
`first_name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
`last_name` varchar(255) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
`total_carbon_footprint` double DEFAULT NULL,
PRIMARY KEY (`id`),
KEY `IDX_8D93D6499E6B1585` (`organisation_id`)
)ENGINE=InnoDB;

--
-- Déchargement des données de la table `user`
--

INSERT INTO `user` (`id`, `organisation_id`, `email`, `roles`, `password`, `first_name`, `last_name`, `total_carbon_footprint`) VALUES
(210, NULL, 'robincby64@gmail.com', '[\"ROLE_USER\"]', '$2y$13$SmZnm6.k.1GUTbcgQiyeHuxTTu6v4qQOhfZW97kQtwTSK34ahuFEG', 'Robin', 'Conchez-Boueytou', 53.65);


--
-- Contraintes pour les tables déchargées
--

--
-- Contraintes pour la table `monitored_website`
--
ALTER TABLE `monitored_website`
ADD CONSTRAINT `FK_7458B0D5A76ED395` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE SET NULL;


--
-- Contraintes pour la table `organisation`
--

ALTER TABLE `organisation`
ADD `admin_id` INT NULL,
ADD CONSTRAINT `FK_organisation_admin`FOREIGN KEY (`admin_id`) REFERENCES `user`(`id`) ON DELETE CASCADE;


--
-- Contraintes pour la table `user`
--
ALTER TABLE `user`
ADD CONSTRAINT `FK_user_organisation`FOREIGN KEY (`organisation_id`) REFERENCES `organisation`(`id`) ON DELETE SET NULL;


COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
