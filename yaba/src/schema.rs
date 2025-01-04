// @generated automatically by Diesel CLI.


#![allow(non_snake_case)]


use serde::Serialize;


// Core tables

diesel::table! {
    Transaction (TransactionID) {
        TransactionID -> Unsigned<Integer>,
        TransactionDate -> Date,
        #[max_length = 200]
        Description -> Varchar,
        Amount -> Decimal,
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize)]
pub enum CategoryTypeEnum {
	Income,
	Expense,
}

diesel::table! {
    use diesel::sql_types::*;
    //use super::sql_types::TransactionCategoryCategoryTypeEnum;
	use super::CategoryTypeEnumMapping;

    TransactionCategory (CategoryID) {
        CategoryID -> Unsigned<Tinyint>,
        #[max_length = 50]
        CategoryName -> Varchar,
        #[max_length = 7]
        CategoryType -> CategoryTypeEnumMapping,//TransactionCategoryCategoryTypeEnum,
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize)]
pub enum AccountTypeEnum {
	Debit,
	Savings,
	Credit,
}

diesel::table! {
    use diesel::sql_types::*;
	use super::AccountTypeEnumMapping;
    //use super::sql_types::PaymentAccountAccountTypeEnum;

    PaymentAccount (AccountID) {
        AccountID -> Unsigned<Tinyint>,
        #[max_length = 50]
        AccountName -> Varchar,
        #[max_length = 7]
        AccountType -> AccountTypeEnumMapping,//PaymentAccountAccountTypeEnum,
    }
}

// Category interface tables

diesel::table! {
    TransactionInstanceCategory (TransactionID) {
        TransactionID -> Unsigned<Integer>,
        CategoryID -> Unsigned<Tinyint>,
    }
}

diesel::table! {
    IncomeCategory (CategoryID) {
        CategoryID -> Unsigned<Tinyint>,
        MonthlyExpected -> Decimal,
    }
}

diesel::table! {
    ExpenseCategory (CategoryID) {
        CategoryID -> Unsigned<Tinyint>,
        MonthlyBudget -> Decimal,
    }
}

// Account integration tables

diesel::table! {
    TransactionAccount (TransactionID) {
        TransactionID -> Unsigned<Integer>,
        AccountID -> Unsigned<Tinyint>,
    }
}

diesel::table! {
    CreditAccount (AccountID) {
        AccountID -> Unsigned<Tinyint>,
        CreditLimit -> Decimal,
    }
}

diesel::table! {
    SavingsAccount (AccountID, MinBalanceForRate) {
        AccountID -> Unsigned<Tinyint>,
        MinBalanceForRate -> Decimal,
        InterestRate -> Decimal,
    }
}

diesel::table! {
    Users (Name) {
        #[max_length = 20]
        Name -> Varchar,
        #[max_length = 64]
        Passkey -> Char,
        BadAttempts -> Unsigned<Tinyint>,
    }
}

// Table joinability

diesel::joinable!(CreditAccount -> PaymentAccount (AccountID));
diesel::joinable!(ExpenseCategory -> TransactionCategory (CategoryID));
diesel::joinable!(IncomeCategory -> TransactionCategory (CategoryID));
diesel::joinable!(SavingsAccount -> PaymentAccount (AccountID));
diesel::joinable!(TransactionAccount -> PaymentAccount (AccountID));
diesel::joinable!(TransactionAccount -> Transaction (TransactionID));
diesel::joinable!(TransactionInstanceCategory -> Transaction (TransactionID));
diesel::joinable!(TransactionInstanceCategory -> TransactionCategory (CategoryID));

diesel::allow_tables_to_appear_in_same_query!(
    CreditAccount,
    ExpenseCategory,
    IncomeCategory,
    PaymentAccount,
    SavingsAccount,
    Transaction,
    TransactionAccount,
    TransactionCategory,
    TransactionInstanceCategory,
	//FIXME:UNCOMMENT?Users,
);

