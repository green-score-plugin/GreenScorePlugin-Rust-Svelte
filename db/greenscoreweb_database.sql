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
(1, 'A/R Lille - Nîmes', 0.24, 'car.png'),
(2, 'A/R Paris - Berlin en TGV', 13.8, 'train-paris-berlin.png'),
(3, 'A/R Paris - Marseille en TGV', 22.7, 'train-paris-marseille.png'),
(4, 'A/R Paris - New York en avion', 0.06, 'new-york.png'),
(5, 'intégrales de Friends', 12.6, 'friends.png'),
(6, 'journées au ski', 2.04, 'ski.png'),
(7, 'marathons des films Harry Potter', 159, 'harry-potter.png'),
(8, 'nuits au camping', 52.6, 'camping.png'),
(9, 'nuits dans un hôtel', 14.5, 'hotel.png'),
(10, 'nuits dans une location', 17.2, 'rental.png'),
(11, 'remplissages de piscines', 13.3, 'swimming.png'),
(12, 'tour de la terre en voiture', 0.01, 'earth.png'),
(13, 'emails', 40620, 'email.png'),
(14, 'go de données', 10521, 'data.png'),
(15, 'recherches sur le web', 1959, 'search.png'),
(16, 'spams', 26748, 'spam.png'),
(17, 'stocker un go de données', 424928, 'data-cloud.png'),
(18, 'heures de streaming vidéo', 1562, 'youtube.png'),
(19, 'heures de visioconférence', 1752, 'video-conference.png'),
(20, 'km en autocar thermique', 3399, 'autocar.png'),
(21, 'km en avion court courrier', 387, 'plane.png'),
(22, 'km en avion long courrier', 658, 'plane.png'),
(23, 'km en bus', 883, 'bus.png'),
(24, 'km en avion moyen courrier', 533, 'plane.png'),
(25, 'km en intercités', 11136, 'intercity.png'),
(26, 'km dans le métro', 22523, 'metro.png'),
(27, 'km en moto thermique', 523, 'motorbike.png'),
(28, 'km en RER', 10225, 'rer.png'),
(29, 'km en scooter', 1311, 'scooter.png'),
(30, 'km en TER', 3611, 'ter.png'),
(31, 'km en TGV', 34130, 'tgv.png'),
(32, 'km en tramway', 23364, 'tramway.png'),
(33, 'km en trottinette électrique', 4016, 'trottinette.png'),
(34, 'km en vélo électrique', 9132, 'bike.png'),
(35, 'km en voiture électrique', 967, 'electric-car.png'),
(36, 'km en voiture thermique', 460, 'car.png'),
(37, 'kg d\'abricots', 114, 'apricot.png'),
(38, 'kg d\'ail', 279, 'garlic.png'),
(39, 'kg d\'ananas', 77.4, 'pineapple.png'),
(40, 'kg d\'artichaut', 25.8, 'artichoke.png'),
(41, 'kg d\'asperge', 64.1, 'asparagus.png'),
(42, 'kg d\'aubergine', 219, 'eggplant.png'),
(43, 'kg d\'avocat', 67.6, 'avocado.png'),
(44, 'kg de banane', 114, 'banana.png'),
(45, 'kg de betterave', 274, 'beetroot.png'),
(46, 'kg de blette', 184, 'chard.png'),
(47, 'kg de brocoli', 111, 'broccoli.png'),
(48, 'kg de carambole', 188, 'carom.png'),
(49, 'kg de cassis', 55.7, 'cassis.png'),
(50, 'kg de carotte', 275, 'carrot.png'),
(51, 'kg de céleri', 148, 'celery.png'),
(52, 'kg de cerise', 74.9, 'cherry.png'),
(53, 'kg de champignon', 203, 'mushroom.png'),
(54, 'kg de châtaigne', 53.2, 'chestnut.png'),
(55, 'kg de chou', 116, 'cabbage.png'),
(56, 'kg de chou de Bruxelles', 174, 'brussels-sprouts.png'),
(57, 'kg de chou-fleur', 136, 'cauliflower.png'),
(58, 'kg de citron', 141, 'lemon.png'),
(59, 'kg de clémentine', 81.9, 'lemon.png'),
(60, 'kg de coing', 185, 'quince.png'),
(61, 'kg de concombre', 211, 'cucumber.png'),
(62, 'kg de courgette', 162, 'zucchini.png'),
(63, 'kg d\'endive', 107, 'endive.png'),
(64, 'kg de fraise', 210, 'strawberry.png'),
(65, 'kg de framboise', 67.8, 'raspberry.png'),
(66, 'kg de kiwi', 102, 'kiwi.png'),
(67, 'kg d\'haricot vert', 242, 'green-bean.png'),
(68, 'kg de laitue', 106, 'salad.png'),
(69, 'kg de maïs', 123, 'corn.png'),
(70, 'kg de mangue', 9.4, 'mango.png'),
(71, 'kg de melon', 107, 'melon.png'),
(72, 'kg de noix de coco', 40.1, 'coconut.png'),
(73, 'kg d\'oignon', 257, 'oignon.png'),
(74, 'kg d\'orange', 158, 'orange.png'),
(75, 'kg de pastèque', 156, 'watermelon.png'),
(76, 'kg de pêche', 168, 'peach.png'),
(77, 'kg de poire', 275, 'pear.png'),
(78, 'kg de poivron', 84.5, 'bell-pepper.png'),
(79, 'kg de pomme', 253, 'apple.png'),
(80, 'kg de raisin', 219, 'grapes.png'),
(81, 'kg de tomate', 172, 'tomato.png'),
(82, 'kg de topinambour', 198, 'topinambour.png'),
(83, 'boxs', 1.22, 'internet-box.png'),
(84, 'casque de réalité virtuelle', 1.38, 'vr-headset.png'),
(85, 'clefs usb', 34.5, 'usb-key.png'),
(86, 'disque durs externes', 8.4, 'hdd.png'),
(87, 'écrans d\'ordinateur', 1.09, 'screen.png'),
(88, 'enceintes connectées', 3.77, 'speakers.png'),
(89, 'ordinateur fixe', 0.33, 'computer.png'),
(90, 'ordinateur portable', 0.52, 'laptop.png'),
(91, 'smartphones', 1.16, 'smartphones.png'),
(92, 'télévision', 0.21, 'tv.png'),
(93, 'kg de beurre', 12.9, 'butter.png'),
(94, 'kg de boeuf', 3.81, 'beef.png'),
(95, 'kg de cheeseburger', 5.79, 'cheeseburger.png'),
(96, 'kg de kebab', 8.57, 'kebab.png'),
(97, 'kg d\'oeufs', 31.6, 'eggs.png'),
(98, 'kg de pâtes', 46.6, 'pasta.png'),
(99, 'repas avec du boeuf', 13.8, 'beefsteak.png'),
(100, 'repas avec du poulet', 63.3, 'chicken.png'),
(101, 'chemises en coton', 7.56, 'shirt.png'),
(102, 'jeans', 3.99, 'jeans.png'),
(103, 'manteau', 0.99, 'jacket.png'),
(104, 'chaussures en cuir', 6.69, 'shoes.png'),
(105, 'sweats en coton', 3.08, 'hoddie.png'),
(106, 'armoire', 0.11, 'cupboard.png'),
(107, 'canapé en textile', 0.56, 'sofa.png'),
(108, 'chaises en bois', 5.37, 'wooden-chair.png'),
(109, 'lit', 0.23, 'bed.png'),
(110, 'table en bois', 1.25, 'table.png'),
(111, 'aspirateurs', 2.11, 'vaccum-cleaner.png'),
(112, 'bouilloires', 2.45, 'kettles.png'),
(113, 'cafetière expresso', 0.47, 'coffee-machine.png'),
(114, 'climatiseur', 0.24, 'clim.png'),
(115, 'four électrique', 0.38, 'four électrique.png'),
(116, 'lave-vaisselle', 0.21, 'dishes-washer.png'),
(117, 'réfrégirateur', 0.31, 'fridge.png'),
(118, 'lave-linge', 0.2, 'washing-machine.png'),
(119, 'litres de bière', 89.3, 'beer.png'),
(120, 'litres de café', 168, 'coffee.png'),
(121, 'litres de soda', 196, 'soda.png'),
(122, 'litres d\'eau du robinet', 747576, 'faucet.png'),
(123, 'litres d\'eau en bouteille', 374, 'water-bottle.png'),
(124, 'litres de vin', 84, 'wine.png'),
(125, 'litres de thé', 2554, 'tee.png'),
(126, 'heures d\'utilisation d\'une LED', 200000, 'led.png'),
(127, 'heures d\'utilisation d\'un ventilateur', 83333, 'ventilateur.png'),
(128, 'min de chargement d\'un smartphone', 1666666, 'chargeur.png'),
(129, 'km en vélo', 588235, 'bike.png');

-- --------------------------------------------------------

--
-- Structure de la table `messenger_messages`
--

DROP TABLE IF EXISTS `messenger_messages`;
CREATE TABLE IF NOT EXISTS `messenger_messages` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `body` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `headers` longtext COLLATE utf8mb4_unicode_ci NOT NULL,
  `queue_name` varchar(190) COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` datetime NOT NULL COMMENT '(DC2Type:datetime_immutable)',
  `available_at` datetime NOT NULL COMMENT '(DC2Type:datetime_immutable)',
  `delivered_at` datetime DEFAULT NULL COMMENT '(DC2Type:datetime_immutable)',
  PRIMARY KEY (`id`),
  KEY `IDX_75EA56E0FB7336F0` (`queue_name`),
  KEY `IDX_75EA56E0E3BD61CE` (`available_at`),
  KEY `IDX_75EA56E016BA31DB` (`delivered_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

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
