use serde;

// AccountRequestsResponse ...
// {
// 	"Data": {
// 		"ConsentId": "A02aff57e-80f9-4964-8548-4c9b17cfaa29",
// 		"Status": "AwaitingAuthorisation",
// 		"CreationDateTime": "2018-10-19T08:36:48+00:00",
// 		"Permissions": [
// 			"ReadAccountsDetail",
// 			"ReadBalances",
// 			"ReadBeneficiariesDetail",
// 			"ReadDirectDebits",
// 			"ReadProducts",
// 			"ReadStandingOrdersDetail",
// 			"ReadTransactionsCredits",
// 			"ReadTransactionsDebits",
// 			"ReadTransactionsDetail"
// 		],
// 		"ExpirationDateTime": "2018-05-02T00:00:00+00:00",
// 		"TransactionFromDateTime": "2017-05-03T00:00:00+00:00",
// 		"TransactionToDateTime": "2018-12-03T00:00:00+00:00"
// 	},
// 	"Risk": {}
// }
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
// #[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct OBReadConsentResponse1 {
    pub data: OBReadConsentResponse1Data,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
// #[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct OBReadConsentResponse1Data {
    pub consent_id: String,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_deserialize() {
        let expected = OBReadConsentResponse1 {
            data: OBReadConsentResponse1Data {
                consent_id: "A02aff57e-80f9-4964-8548-4c9b17cfaa29".to_string(),
            },
        };
        let json = r#"
{
    "Data": {
        "ConsentId": "A02aff57e-80f9-4964-8548-4c9b17cfaa29",
        "Status": "AwaitingAuthorisation",
        "CreationDateTime": "2018-10-19T08:36:48+00:00",
        "Permissions": [
            "ReadAccountsDetail",
            "ReadBalances",
            "ReadBeneficiariesDetail",
            "ReadDirectDebits",
            "ReadProducts",
            "ReadStandingOrdersDetail",
            "ReadTransactionsCredits",
            "ReadTransactionsDebits",
            "ReadTransactionsDetail"
        ],
        "ExpirationDateTime": "2018-05-02T00:00:00+00:00",
        "TransactionFromDateTime": "2017-05-03T00:00:00+00:00",
        "TransactionToDateTime": "2018-12-03T00:00:00+00:00"
    },
    "Risk": {}
}
        "#;
        let actual = serde_json::from_str::<OBReadConsentResponse1>(json).unwrap();
        let serialized = serde_json::to_string(&expected).unwrap();
        println!("serialized = {}", serialized);
        println!("actual={:?}, expected={:?}", actual, expected);
        assert_eq!(actual, expected);
    }
}
