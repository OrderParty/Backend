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
* Notification Center
    * Show notifications


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
> table for license (activation from/to date etc.)  
> tax for items?  
> Discount for items (resetable)

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
    * Scope?
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
    * Completed
* PaymentItem
    * PaymentId
    * ItemId
    * Quantity
* Table
    * EventId
    * Number?
    * Name
* Order
    * EventId
    * TableId
    * WaiterId
    * Date/Time
* Payment
    * EventId
    * TableId
    * WaiterId
    * Date/Time
* Discount
    * Name
    * Value
* Setting
    * EventId
    * Key
    * Value
* Notfication
    * TargetWaiterId (brotcast when null)
    * Name
    * Description
    * StartDate


### Endpoints

* Login
* Table
    * GetTables
* Item
    * GetItems
    * UpdateItemStock
* Discount
    * GetDiscounts
* Order
    * GetOrders
    * AddOrder
    * CancelOrderItem
* Payment
    * GetUnpaidOrderItems
    * AddPayment
* Setting
    * GetSettings
* Notification
    * GetNotifications (serverside event)
* OrderItem
    * UpdateOrderItem
    * CancelOrderItem

### Endpoints - Admin

* Login
* Table
    * AddTable
    * UpdateTable
    * DeleteTable
* Discount
    * AddDiscount
    * UpdateDiscount
    * DeleteDiscount
* Item
    * AddItem
    * UpdateItem
    * DeleteItem
* Setting
    * SetSetting