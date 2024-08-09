-- Your SQL goes here
CREATE TABLE `notifications`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`event_id` INTEGER NOT NULL,
	`waiter_id` INTEGER,
	`name` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`timestamp` TIMESTAMP NOT NULL,
	FOREIGN KEY (`event_id`) REFERENCES `events`(`id`),
	FOREIGN KEY (`waiter_id`) REFERENCES `waiters`(`id`)
);

CREATE TABLE `settings`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`event_id` INTEGER NOT NULL,
	`key` TEXT NOT NULL,
	`value` TEXT NOT NULL,
	FOREIGN KEY (`event_id`) REFERENCES `events`(`id`)
);

CREATE TABLE `discounts`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`event_id` INTEGER NOT NULL,
	`name` TEXT NOT NULL,
	`value` INTEGER NOT NULL,
	FOREIGN KEY (`event_id`) REFERENCES `events`(`id`)
);

CREATE TABLE `events`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`access_token` TEXT NOT NULL,
	`admin_access_token` TEXT NOT NULL
);

CREATE TABLE `order_items`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`order_id` INTEGER NOT NULL,
	`item_id` INTEGER NOT NULL,
	`payment_id` INTEGER,
	`info` TEXT NOT NULL,
	`price` FLOAT NOT NULL,
	`discount` FLOAT NOT NULL,
	`completed` BOOL NOT NULL,
	FOREIGN KEY (`order_id`) REFERENCES `orders`(`id`),
	FOREIGN KEY (`item_id`) REFERENCES `items`(`id`),
	FOREIGN KEY (`payment_id`) REFERENCES `payments`(`id`)
);

CREATE TABLE `orders`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`table_id` INTEGER NOT NULL,
	`timestamp` TIMESTAMP NOT NULL,
	FOREIGN KEY (`table_id`) REFERENCES `tables`(`id`)
);

CREATE TABLE `waiters`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`event_id` INTEGER NOT NULL,
	`name` TEXT NOT NULL,
	`access_pin` TEXT NOT NULL,
	`is_active` BOOL NOT NULL,
	`can_accept_payment` BOOL NOT NULL,
	`scope` TEXT NOT NULL,
	FOREIGN KEY (`event_id`) REFERENCES `events`(`id`)
);

CREATE TABLE `order_item_extras`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`order_item_id` INTEGER NOT NULL,
	`extra_id` INTEGER NOT NULL,
	`amount` INTEGER NOT NULL,
	FOREIGN KEY (`order_item_id`) REFERENCES `order_items`(`id`),
	FOREIGN KEY (`extra_id`) REFERENCES `extras`(`id`)
);

CREATE TABLE `payments`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`table_id` INTEGER NOT NULL,
	`waiter_id` INTEGER NOT NULL,
	`discount_id` INTEGER NOT NULL,
	`price` FLOAT NOT NULL,
	`timestamp` TIMESTAMP NOT NULL,
	FOREIGN KEY (`table_id`) REFERENCES `tables`(`id`),
	FOREIGN KEY (`waiter_id`) REFERENCES `waiters`(`id`),
	FOREIGN KEY (`discount_id`) REFERENCES `discounts`(`id`)
);

CREATE TABLE `items`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`event_id` INTEGER NOT NULL,
	`name` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`image` TEXT NOT NULL,
	`price` FLOAT NOT NULL,
	`discounted_price` FLOAT NOT NULL,
	`stock` INTEGER NOT NULL,
	FOREIGN KEY (`event_id`) REFERENCES `events`(`id`)
);

CREATE TABLE `tables`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`event_id` INTEGER NOT NULL,
	`name` TEXT NOT NULL,
	FOREIGN KEY (`event_id`) REFERENCES `events`(`id`)
);

CREATE TABLE `extras`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`item_id` INTEGER NOT NULL,
	`name` TEXT NOT NULL,
	`price` FLOAT NOT NULL,
	`max_amount` INTEGER NOT NULL,
	FOREIGN KEY (`item_id`) REFERENCES `items`(`id`)
);

