-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Hôte : 127.0.0.1:3306
-- Généré le : lun. 16 mars 2026 à 12:26
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
    ) ENGINE=InnoDB AUTO_INCREMENT=134 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

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
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

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

-- --------------------------------------------------------

--
-- Structure de la table `user`
--

DROP TABLE IF EXISTS `user`;
CREATE TABLE IF NOT EXISTS `user` (
                                      `id` int NOT NULL AUTO_INCREMENT,
                                      `organisation_id` int DEFAULT NULL,
                                      `service_id` int DEFAULT NULL,
                                      `email` varchar(180) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
    `first_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `last_name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci DEFAULT NULL,
    `est_admin` tinyint(1) NOT NULL DEFAULT '0',
    `total_carbon_footprint` double NOT NULL DEFAULT '0',
    PRIMARY KEY (`id`),
    KEY `IDX_8D93D6499E6B1585` (`organisation_id`),
    KEY `id_service` (`service_id`)
    ) ENGINE=InnoDB AUTO_INCREMENT=1005 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- --------------------------------------------------------

--
-- Structure de la table `organisation_user`
--

DROP TABLE IF EXISTS `organisation_user`;
CREATE TABLE IF NOT EXISTS `organisation_user` (
  `user_id` int(11) NOT NULL,
  `organisation_id` int(11) NOT NULL,
  `est_admin` tinyint(1) NOT NULL DEFAULT 0,
  PRIMARY KEY (`user_id`,`organisation_id`),
  KEY `IDX_ORG_USER_ORG` (`organisation_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


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
-- Contraintes pour les tables déchargées
--

--
-- Contraintes pour la table `monitored_website`
--
ALTER TABLE `monitored_website`
    ADD CONSTRAINT `FK_7458B0D5A76ED395` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE SET NULL;

--
-- Contraintes pour la table `service`
--
ALTER TABLE `service`
    ADD CONSTRAINT `service_ibfk_1` FOREIGN KEY (`id_organisation`) REFERENCES `organisation` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Contraintes pour la table `user`
--
ALTER TABLE `user`
    ADD CONSTRAINT `FK_user_organisation` FOREIGN KEY (`organisation_id`) REFERENCES `organisation` (`id`) ON DELETE SET NULL,
  ADD CONSTRAINT `user_ibfk_1` FOREIGN KEY (`service_id`) REFERENCES `service` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

--
-- Contraintes pour la table `organisation_user`
--
ALTER TABLE `organisation_user`
  ADD CONSTRAINT `FK_ORG_USER_USER` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `FK_ORG_USER_ORG` FOREIGN KEY (`organisation_id`) REFERENCES `organisation` (`id`) ON DELETE CASCADE;

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
