// @generated automatically by Diesel CLI.


use serde::Serialize;


diesel::table! {
    CreditAccount (AccountID) {
        AccountID -> Unsigned<Tinyint>,
        CreditLimit -> Decimal,
    }
}

diesel::table! {
    ExpenseCategory (CategoryID) {
        CategoryID -> Unsigned<Tinyint>,
        MonthlyBudget -> Decimal,
    }
}

diesel::table! {
    IncomeCategory (CategoryID) {
        CategoryID -> Unsigned<Tinyint>,
        MonthlyExpected -> Decimal,
    }
}

diesel::table! {
    Job (JobID) {
        JobID -> Unsigned<Tinyint>,
        #[max_length = 50]
        JobName -> Varchar,
        Wage -> Decimal,
        ExpectedMonthlyHours -> Unsigned<Tinyint>,
        ExpectedMonthlyMissHours -> Unsigned<Tinyint>,
    }
}

diesel::table! {
    JobIncome (CategoryID) {
        CategoryID -> Unsigned<Tinyint>,
        JobID -> Unsigned<Tinyint>,
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

diesel::table! {
    SavingsAccount (AccountID, MinBalanceForRate) {
        AccountID -> Unsigned<Tinyint>,
        MinBalanceForRate -> Decimal,
        InterestRate -> Decimal,
    }
}

diesel::table! {
    Transaction (TransactionID) {
        TransactionID -> Unsigned<Integer>,
        TransactionDate -> Date,
        #[max_length = 200]
        Description -> Varchar,
        Amount -> Decimal,
    }
}

diesel::table! {
    TransactionAccount (TransactionID) {
        TransactionID -> Unsigned<Integer>,
        AccountID -> Unsigned<Tinyint>,
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

diesel::table! {
    TransactionInstanceCategory (TransactionID) {
        TransactionID -> Unsigned<Integer>,
        CategoryID -> Unsigned<Tinyint>,
    }
}

diesel::joinable!(CreditAccount -> PaymentAccount (AccountID));
diesel::joinable!(ExpenseCategory -> TransactionCategory (CategoryID));
diesel::joinable!(IncomeCategory -> TransactionCategory (CategoryID));
diesel::joinable!(JobIncome -> IncomeCategory (CategoryID));
diesel::joinable!(JobIncome -> Job (JobID));
diesel::joinable!(SavingsAccount -> PaymentAccount (AccountID));
diesel::joinable!(TransactionAccount -> PaymentAccount (AccountID));
diesel::joinable!(TransactionAccount -> Transaction (TransactionID));
diesel::joinable!(TransactionInstanceCategory -> Transaction (TransactionID));
diesel::joinable!(TransactionInstanceCategory -> TransactionCategory (CategoryID));

diesel::allow_tables_to_appear_in_same_query!(
    CreditAccount,
    ExpenseCategory,
    IncomeCategory,
    Job,
    JobIncome,
    PaymentAccount,
    SavingsAccount,
    Transaction,
    TransactionAccount,
    TransactionCategory,
    TransactionInstanceCategory,
);
