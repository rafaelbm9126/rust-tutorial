pub mod mods_template {
    // --------------- //
    use crate::helpers::models::csv::csv::mods_csv::ErrorLoadDetail;
    use crate::template::data::model::dt_template::DTemplate;
    use bson::oid::ObjectId;
    // --------------- //

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Template {
        pub Funnel_Status: String,
        pub DecisionMaker: String,
        pub LastName: String,
        pub Company_Area: String,
        pub Company_Position: String,
        pub Personal_Mail: String,
        pub Personal_CompanyMail: String,
        pub CellPhone_Whatsapp: String,
        pub Skype_User: String,
        pub HangOut_User: String,
        pub Linkedin_URL: String,
        pub Picture_URL: String,
        pub Facebook_URL: String,
        pub Instagram_URL: String,
        pub Interests: String,
        pub Sex: String,
        pub NSE: String,
        pub Birthday: String,
        pub Media_Consumption: String,
        pub Company_LinkedIn_URL: String,
        pub Company_Name: String,
        pub Potential_Size: String,
        pub Company_Sector: String,
        pub Company_Products: String,
        pub Web_URL: String,
        pub Company_Phone: String,
        pub Sucursal_Location: String,
        pub City: String,
        pub State: String,
        pub Country: String,
        pub NextPurchase_Date: String,
        pub Satisfaction_DM: String,
        pub Operator_MailID: String,
        pub Countable_Number: String,
        pub DM_Countable: String,
        pub Personal_CountableMail: String,
        pub CellPhone_Countable: String,
        pub Payment_Date: String,
        pub Frecuency: String,
        pub Payment_Method: String,
        pub Payment_Ammount: String,
        pub Status_Countable: String,
        pub Payment_Description: String,
    }

    impl Template {
        pub fn parse (self) -> DTemplate {
            DTemplate {
                id                    : ObjectId::new().unwrap(),
                funnel_status         : self.Funnel_Status,
                decision_maker        : self.DecisionMaker,
                last_name             : self.LastName,
                company_area          : self.Company_Area,
                company_position      : self.Company_Position,
                personal_mail         : self.Personal_Mail,
                personal_companymail  : self.Personal_CompanyMail,
                cellphone_whatsapp    : self.CellPhone_Whatsapp,
                skype_user            : self.Skype_User,
                hangout_user          : self.HangOut_User,
                linkedin_url          : self.Linkedin_URL,
                picture_url           : self.Picture_URL,
                facebook_url          : self.Facebook_URL,
                instagram_url         : self.Instagram_URL,
                interests             : self.Interests,
                sex                   : self.Sex,
                nse                   : self.NSE,
                birthday              : self.Birthday,
                media_consumption     : self.Media_Consumption,
                company_linkedin_url  : self.Company_LinkedIn_URL,
                company_name          : self.Company_Name,
                potential_size        : self.Potential_Size,
                company_sector        : self.Company_Sector,
                company_products      : self.Company_Products,
                web_url               : self.Web_URL,
                company_phone         : self.Company_Phone,
                sucursal_location     : self.Sucursal_Location,
                city                  : self.City,
                state                 : self.State,
                country               : self.Country,
                nextpurchase_date     : self.NextPurchase_Date,
                satisfaction_dm       : self.Satisfaction_DM,
                operator_mailid       : self.Operator_MailID,
                countable_number      : self.Countable_Number,
                dm_countable          : self.DM_Countable,
                personal_countablemail: self.Personal_CountableMail,
                cellphone_countable   : self.CellPhone_Countable,
                payment_date          : self.Payment_Date,
                frecuency             : self.Frecuency,
                payment_method        : self.Payment_Method,
                payment_ammount       : self.Payment_Ammount,
                status_countable      : self.Status_Countable,
                payment_description   : self.Payment_Description,
            }
        }
    }

    #[derive(Serialize)]
    pub struct CustomResponse {
        pub message : String,
        pub inserted: u64,
        pub fail    : Vec<Option<ErrorLoadDetail>>,
    }
}