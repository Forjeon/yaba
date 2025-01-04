-- Create tables

	-- Core tables

CREATE TABLE IF NOT EXISTS Transaction (
	TransactionID INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	TransactionDate DATE NOT NULL,
	Description VARCHAR(200) NOT NULL,
	Amount DECIMAL(10,2) NOT NULL,
	UNIQUE (TransactionDate, Description, Amount)
);

CREATE TABLE IF NOT EXISTS TransactionCategory (
	CategoryID TINYINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	CategoryName VARCHAR(50) NOT NULL UNIQUE,
	CategoryType ENUM('income','expense') NOT NULL
);

CREATE TABLE IF NOT EXISTS PaymentAccount (
	AccountID TINYINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
	AccountName VARCHAR(50) NOT NULL UNIQUE,
	AccountType ENUM('debit','savings','credit') NOT NULL
);

	-- Category interface tables

CREATE TABLE IF NOT EXISTS TransactionInstanceCategory (
	TransactionID INT UNSIGNED NOT NULL PRIMARY KEY,
	CategoryID TINYINT UNSIGNED NOT NULL,
	FOREIGN KEY (TransactionID) REFERENCES Transaction(TransactionID) ON DELETE CASCADE,
	FOREIGN KEY (CategoryID) REFERENCES TransactionCategory(CategoryID)
);

CREATE TABLE IF NOT EXISTS IncomeCategory (
	CategoryID TINYINT UNSIGNED NOT NULL PRIMARY KEY,
	MonthlyExpected DECIMAL(10,2) NOT NULL,
	FOREIGN KEY (CategoryID) REFERENCES TransactionCategory(CategoryID) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ExpenseCategory (
	CategoryID TINYINT UNSIGNED NOT NULL PRIMARY KEY,
	MonthlyBudget DECIMAL(10,2) NOT NULL,
	FOREIGN KEY (CategoryID) REFERENCES TransactionCategory(CategoryID) ON DELETE CASCADE
);

	-- Account interface tables

CREATE TABLE IF NOT EXISTS TransactionAccount (
	TransactionID INT UNSIGNED NOT NULL PRIMARY KEY,
	AccountID TINYINT UNSIGNED NOT NULL,
	FOREIGN KEY (TransactionID) REFERENCES Transaction(TransactionID) ON DELETE CASCADE,
	FOREIGN KEY (AccountID) REFERENCES PaymentAccount(AccountID)
);

CREATE TABLE IF NOT EXISTS CreditAccount (
	AccountID TINYINT UNSIGNED NOT NULL PRIMARY KEY,
	CreditLimit DECIMAL(10,2) NOT NULL,
	FOREIGN KEY (AccountID) REFERENCES PaymentAccount(AccountID)
);

CREATE TABLE IF NOT EXISTS SavingsAccount (
	AccountID TINYINT UNSIGNED NOT NULL,
	MinBalanceForRate DECIMAL(10,2) NOT NULL,
	InterestRate DECIMAL(5,4) NOT NULL,
	PRIMARY KEY (AccountID, MinBalanceForRate),
	FOREIGN KEY (AccountID) REFERENCES PaymentAccount(AccountID)
);

	-- Users table

CREATE TABLE IF NOT EXISTS Users (
	Name VARCHAR(20) NOT NULL PRIMARY KEY,
	Passkey CHAR(64) NOT NULL,
	BadAttempts TINYINT UNSIGNED NOT NULL
);


-- Populate categories

INSERT INTO TransactionCategory(CategoryName, CategoryType) VALUES
("Paycheck J", 'income'),
("Paycheck E", 'income'),
("Misc Income", 'income'),
("Tithing", 'expense'),
("Savings", 'expense'),
("Roth IRA", 'expense'),
("Rent", 'expense'),
("Electric Bill", 'expense'),
("Gas Bill", 'expense'),
("Internet Bill", 'expense'),
("Groceries", 'expense'),
("Car", 'expense'),
("Phone Bill", 'expense'),
("Medical", 'expense'),
("Therapy", 'expense'),
("Date", 'expense'),
("Eat Out", 'expense'),
("Friend Night", 'expense'),
("Fun", 'expense'),
("School", 'expense'),
("Travel", 'expense'),
("Gift", 'expense'),
("House", 'expense'),
("Emergency", 'expense'),
("Misc Expense", 'expense');

	-- Income categories

INSERT INTO Job(JobName, Wage, ExpectedMonthlyHours, ExpectedMonthlyMissHours) VALUES
("Silent Falcon UAS Technologies", 25.00, 30, 12),
("Custom Stickers", 16.00, 36, 16);

INSERT INTO IncomeCategory(CategoryID, MonthlyExpected) VALUES
(1, 450.00),
(2, 320.00),
(3, 0.00);

INSERT INTO JobIncome(CategoryID, JobID) VALUES
(1, 1),
(2, 2);

	-- Expense categories

INSERT INTO ExpenseCategory(CategoryID, MonthlyBudget) VALUES
(4, 77.00),
(5, 154.00),
(6, 50.00),
(7, 1087.00),
(8, 35.00),
(9, 25.00),
(10, 10.00),
(11, 350.00),
(12, 50.00),
(13, 16.00),
(14, 100.00),
(15, 50.00),
(16, 120.00),
(17, 30.00),
(18, 100.00),
(19, 60.00),
(20, 0.00),
(21, 0.00),
(22, 0.00),
(23, 0.00),
(24, 0.00),
(25, 0.00);


-- Populate accounts

INSERT INTO PaymentAccount(AccountName, AccountType) VALUES
("Joint Checking", 'debit'),
("Joint Savings", 'savings'),
("Credit Card J", 'credit'),
("Credit Card E", 'credit');

INSERT INTO SavingsAccount(AccountID, MinBalanceForRate, InterestRate) VALUES
(2, 1000.00, 0.0150),
(2, 10000.00, 0.0160),
(2, 100000.00, 0.0250),
(2, 250000.00, 0.0330),
(2, 500000.00, 0.0375);

INSERT INTO CreditAccount(AccountID, CreditLimit) VALUES
(3, 1800.00),
(4, 1500.00);

