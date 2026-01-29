SET FOREIGN_KEY_CHECKS = 0;

CREATE TABLE IF NOT EXISTS `advice` (
                                        `id` int NOT NULL AUTO_INCREMENT,
                                        `is_dev` tinyint(1) NOT NULL,
    `advice` longtext NOT NULL,
    `title` varchar(100) NOT NULL,
    `icon` varchar(100) NOT NULL,
    PRIMARY KEY (`id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `equivalent` (
                                            `id` int NOT NULL AUTO_INCREMENT,
                                            `name` varchar(255) NOT NULL,
    `equivalent` double NOT NULL,
    `icon_thumbnail` varchar(255) NOT NULL,
    PRIMARY KEY (`id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `sessions` (
                                          `id` varchar(128) NOT NULL,
    `data` longblob NOT NULL,
    `expires_at` bigint DEFAULT NULL,
    PRIMARY KEY (`id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 2. Création de User et Organisation (Dépendances croisées)
-- On crée les tables d'abord, on ajoutera les contraintes à la fin
CREATE TABLE IF NOT EXISTS `organisation` (
                                              `id` int NOT NULL AUTO_INCREMENT,
                                              `organisation_name` varchar(255) NOT NULL,
    `organisation_code` varchar(20) NOT NULL,
    `city` varchar(255) DEFAULT NULL,
    `siret` varchar(14) DEFAULT NULL,
    `admin_id` int DEFAULT NULL,
    PRIMARY KEY (`id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `user` (
                                      `id` int NOT NULL AUTO_INCREMENT,
                                      `organisation_id` int DEFAULT NULL,
                                      `email` varchar(180) NOT NULL,
    `roles` longtext NOT NULL,
    `password` varchar(255) NOT NULL,
    `first_name` varchar(255) DEFAULT NULL,
    `last_name` varchar(255) DEFAULT NULL,
    `total_carbon_footprint` double DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `IDX_user_organisation` (`organisation_id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 3. Tables avec clés étrangères vers User
CREATE TABLE IF NOT EXISTS `monitored_website` (
                                                   `id` int NOT NULL AUTO_INCREMENT,
                                                   `user_id` int DEFAULT NULL,
                                                   `url_domain` varchar(255) DEFAULT NULL,
    `url_full` longtext,
    `queries_quantity` int DEFAULT NULL,
    `carbon_footprint` double DEFAULT NULL,
    `data_transferred` double DEFAULT NULL,
    `resources` longtext,
    `loading_time` double DEFAULT NULL,
    `country` varchar(255) DEFAULT NULL,
    `creation_date` datetime DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `IDX_monitored_user` (`user_id`),
    CONSTRAINT `FK_monitored_user` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE SET NULL
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `user_equivalent` (
                                                 `user_id` int NOT NULL,
                                                 `equivalent_id` int NOT NULL,
                                                 PRIMARY KEY (`user_id`,`equivalent_id`),
    CONSTRAINT `FK_ue_user` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE,
    CONSTRAINT `FK_ue_equivalent` FOREIGN KEY (`equivalent_id`) REFERENCES `equivalent` (`id`) ON DELETE CASCADE
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 4. Ajout des contraintes circulaires entre User et Organisation
ALTER TABLE `organisation` ADD CONSTRAINT `FK_org_admin` FOREIGN KEY (`admin_id`) REFERENCES `user` (`id`) ON DELETE SET NULL;
ALTER TABLE `user` ADD CONSTRAINT `FK_user_org` FOREIGN KEY (`organisation_id`) REFERENCES `organisation` (`id`) ON DELETE SET NULL;

-- 5. Insertion des données de référence (Advice & Equivalents)
INSERT INTO `advice` (`id`, `is_dev`, `advice`, `title`, `icon`) VALUES
                                                                     (1, 1, 'Optimisez vos requêtes SQL...', 'Optimisez vos requêtes', 'fa-solid fa-database'),
                                                                     (2, 1, 'Minifiez et compressez...', 'Minifiez vos fichiers', 'fa-brands fa-css'),
                                                                     (21, 0, 'Réglez la luminosité...', 'Écran moins lumineux', 'fa-solid fa-desktop');
-- (Ajoute ici le reste de tes INSERT pour que les tests aient accès aux conseils)

INSERT INTO `equivalent` (`id`, `name`, `equivalent`, `icon_thumbnail`) VALUES
                                                                            (1, 'A/R Lille - Nîmes', 0.004, 'car.png'),
                                                                            (3, 'emails', 2000, 'email.png');

SET FOREIGN_KEY_CHECKS = 1;