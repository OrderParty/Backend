-- EVENTS
INSERT INTO events VALUES (1, 'default', '', '');

-- SETTINGS
INSERT INTO settings VALUES (1, 1, 'theme', 'light');

-- DISCOUNTS
INSERT INTO discounts VALUES (1, 1, 'Friends and Family', 10);

-- WAITERS
INSERT INTO waiters VALUES (1, 1, 'John Doe', '1234', true, true, 'food');

-- NOTIFICATIONS
INSERT INTO notifications VALUES (1, 1, 1, 'Test Notification', 'This is a test Notification', '2024-09-10 21:52:01');

-- TABLES
INSERT INTO tables VALUES (1, 1, 'Table 1');

-- ORDERS
INSERT INTO orders VALUES (1, 1, 1, '2024-09-10 21:52:01');

-- ITEMS
-- TODO: add img
INSERT INTO items VALUES (1, 1, 'Test Item', 'This is a test item', '', 15.5, 10.5, 1000);

-- PAYMENTS
INSERT INTO payments VALUES (1, 1, 1, 1, 15, '2024-09-10 21:52:01');

-- ORDER-ITEMS
INSERT INTO order_items VALUES (1, 1, 1, 1, 'This is a test order item', 15.5, 0, false);

-- EXTRAS
INSERT INTO extras VALUES (1, 1, 'Görk', 1, 10);

-- ORDER-ITEM-EXTRAS
INSERT INTO order_item_extras VALUES (1, 1, 1, 7);