# OrderParty - Features

## Frontend

* Switch between views
* Settings
    * Base URL
    * API Token
    * View specific

## Frontend - Kitchen

* Order Overview
    * Item Overview
    * Complete single item
    * Complete order

    * Cancel single item
    * Cancel Order

* Item Overview
    * Show stock
    * Add/Remove/Set amount
    * Set out of stock

## Frontend - POS

> Cascading card selector

* Waiter Login
    * with PIN
* Table selection (Table master)
    * Using cascading selector
    * Option to jump to payment (If delayed payment was choosen)
* Item selection
    * Using cascading selector
    * Item amount selection
* Order overview
    * Discound using cascading selector
    * Choose instant or delayed payment.
* Payment
    * If instant pay, auto select items from order
    * View to select unpaid items
    * Calc change (usage optional)
        * Suggest usefull amount
        * Manual amount input







## Frontend - Admin

* Table tree editor
* Item tree editor
* Discount tree editor
* Global settings
    * delayed payment

* Data view
* Data export


## Backend

> Notes:
> * table for license (activation from/to date etc.)
> tax for items?

### Models

* Event
    * Name
    * Admin access token
    * Access token
* Waiter
    * EventId
    * Name
    * Access PIN
    * IsActive
    * Can accept payment (only when delayed payment enabled)
* Item
    * EventId
    * Name
    * Description
    * Icon/Image
    * Price
    * Stock
* OrderItem
    * OrderId
    * ItemId
    * Info (Extra görk)
    * Quantity
* PaymentItem
    * PaymentId
    * ItemId
    * Quantity
* Desk
    * EventId
    * Number?
    * Name
* Order
    * EventId
    * DeskId
    * WaiterId
    * Date/Time
* Payment
    * EventId
    * DeskId
    * WaiterId
    * Date/Time
* Settings
    * EventId
    * Key
    * Value




### Endpoints



    