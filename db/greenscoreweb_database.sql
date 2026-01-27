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
(1, 1, 'Optimisez vos requêtes SQL pour éviter les opérations inutiles.', 'Optimisez vos requêtes', 'fa-solid fa-database'),
(2, 1, 'Minifiez et compressez les fichiers CSS et JavaScript.', 'Minifiez vos fichiers', 'fa-brands fa-css'),
(3, 1, 'Implémentez un système de cache efficace côté serveur.', 'Cache serveur efficace', 'fa-solid fa-server'),
(4, 1, 'Réduisez le nombre de requêtes HTTP en regroupant les fichiers.', 'Regroupez vos fichiers', 'fa-solid fa-file'),
(5, 1, 'Utilisez des images dans des formats modernes comme WebP.', 'Utilisez WebP', 'fa-solid fa-image'),
(6, 1, 'Adoptez le chargement différé (lazy loading) pour les images et les scripts.', 'Activez le lazy loading', 'fa-solid fa-image'),
(7, 1, 'Évitez les boucles infinies ou les calculs inutiles côté serveur.', 'Évitez les boucles', 'fa-solid fa-rotate-left'),
(8, 1, 'Déployez votre application sur des serveurs écologiques.', 'Serveurs écologiques', 'fa-solid fa-server'),
(9, 1, 'Utilisez des CDN pour distribuer vos contenus statiques.', 'CDN pour contenus', 'fa-solid fa-chart-simple'),
(10, 1, 'Surveillez et réduisez l\'utilisation de mémoire et CPU dans vos scripts.', 'Réduisez mémoire & CPU', 'fa-solid fa-microchip'),
(11, 1, 'Concevez des algorithmes plus efficaces pour économiser des ressources.', 'Algorithmes optimisés', 'fa-solid fa-chart-diagram'),
(12, 1, 'Réduisez la taille des cookies et limitez leur utilisation.', 'Cookies allégés', 'fa-solid fa-cookie'),
(13, 1, 'Implémentez des stratégies de mise en cache HTTP (ETag, Cache-Control).', 'Cache HTTP stratégique', 'fa-solid fa-server'),
(14, 1, 'Utilisez des frameworks ou bibliothèques légères.', 'Frameworks légers', 'fa-solid fa-book'),
(15, 1, 'Choisissez des bases de données adaptées à vos besoins pour éviter la surcharge.', 'Base de données adaptée', 'fa-solid fa-database'),
(16, 1, 'Testez régulièrement les performances de votre code.', 'Testez vos performances', 'fa-solid fa-code'),
(17, 1, 'Optez pour des solutions cloud avec une empreinte carbone réduite.', 'Cloud écoresponsable', 'fa-solid fa-cloud-arrow-up'),
(18, 1, 'Désactivez les logs en mode production pour réduire la charge.', 'Désactivez les logs', 'fa-solid fa-file-lines'),
(19, 1, 'Générez les rapports ou analyses hors ligne si possible.', 'Rapports hors ligne', 'fa-solid fa-bug'),
(20, 1, 'Faites des audits réguliers pour détecter les ressources inutilisées.', 'Auditez vos ressources', 'fa-solid fa-network-wired'),
(21, 0, 'Réglez la luminosité de votre écran pour économiser de l\'énergie.', 'Écran moins lumineux', 'fa-solid fa-desktop'),
(22, 0, 'Fermez les onglets inutilisés dans votre navigateur.', 'Fermez les onglets', 'fa-solid fa-window-restore'),
(23, 0, 'Utilisez un bloqueur de publicités pour réduire la charge réseau.', 'Bloquez les pubs', 'fa-solid fa-bullhorn'),
(24, 0, 'Privilégiez les moteurs de recherche écologiques comme Ecosia.', 'Moteurs écolos', 'fa-solid fa-leaf'),
(25, 0, 'Désactivez les vidéos en lecture automatique.', 'Stop aux vidéos auto', 'fa-solid fa-photo-film'),
(26, 0, 'Téléchargez des fichiers uniquement si nécessaire.', 'Téléchargez malin', 'fa-solid fa-file'),
(27, 0, 'Effacez régulièrement les caches et cookies de votre navigateur.', 'Videz caches & cookies', 'fa-solid fa-cookie'),
(28, 0, 'Limitez l\'utilisation des extensions de navigateur gourmandes.', 'Limitez les extensions', 'fa-solid fa-puzzle-piece'),
(29, 0, 'Privilégiez le mode économie de données sur les appareils mobiles.', 'Mode économie activé', 'fa-solid fa-leaf'),
(30, 0, 'Consultez les sites optimisés pour mobile pour réduire la consommation de données.', 'Sites mobiles optimisés', 'fa-solid fa-mobile'),
(31, 0, 'Regardez des vidéos en basse résolution si la haute définition n\'est pas nécessaire.', 'Vidéo en basse qualité', 'fa-solid fa-photo-film'),
(32, 0, 'Déconnectez-vous des comptes inutilisés pendant la navigation.', 'Déconnectez les comptes', 'fa-solid fa-right-from-bracket'),
(33, 0, 'Utilisez un navigateur léger comme Firefox Focus pour les tâches simples.', 'Navigateur ultra léger', 'fa-solid fa-window-maximize'),
(34, 0, 'Téléchargez des contenus en Wi-Fi au lieu d\'utiliser les données mobiles.', 'Wi-Fi avant tout', 'fa-solid fa-wifi'),
(35, 0, 'Évitez les sites web surchargés de publicités.', 'Évitez les pubs lourdes', 'fa-solid fa-bullhorn'),
(36, 0, 'Planifiez vos recherches web pour éviter de multiplier les requêtes.', 'Recherches planifiées', 'fa-solid fa-globe'),
(37, 0, 'Privilégiez les versions texte des articles lorsque c\'est possible.', 'Articles en mode texte', 'fa-solid fa-text-height'),
(38, 0, 'Partagez les ressources (documents, vidéos) via des liens plutôt qu\'en pièce jointe.', 'Partagez via liens', 'fa-solid fa-link'),
(39, 0, 'Désactivez les notifications push non essentielles sur les sites web.', 'Désactivez notifications', 'fa-solid fa-bell'),
(40, 0, 'Fermez les applications ouvertes en arrière-plan pour économiser des ressources.', 'Fermez les apps inutiles', 'fa-solid fa-square-xmark');

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
(9, 'data.equivalent.laptop', 0.004, 'laptop.png'),
(10, 'data.equivalent.tap_water', 7692, 'faucet.png'),
(11, 'data.equivalent.paris_ny', 0.0007, 'new-york.png');

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
