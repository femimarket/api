# ApiAction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** |  | 
**type** | **String** |  | 
**falRequestId** | **String** |  | 
**file** | **String** | base64 return | 
**prompt** | **String** |  | 
**audio** | **String** | input image as base64 — data URI (web) or raw base64 (android/ios), empty if unused; type detected server-side | 
**comfyRequestId** | **String** |  | 
**image** | **String** | input image as base64 — data URI (web) or raw base64 (android/ios), empty if unused; type detected server-side | 
**messages** | [ApiChatMessage] |  | 
**amountCents** | **Int64** |  | 
**credit** | **Int64** |  | 
**loaded** | **Bool** |  | 
**paymentUrl** | **String** |  | 
**stripePaymentIntentId** | **String** |  | 
**stripeSessionId** | **String** |  | 
**currency** | **String** |  | 
**jws** | **String** |  | 
**price** | **Int64** |  | 
**productId** | **String** |  | 
**transactionId** | **String** |  | 
**orderId** | **String** |  | [optional] 
**packageName** | **String** |  | 
**purchaseToken** | **String** |  | 
**characters** | [CharacterAlignment] | return | 
**lyrics** | **String** |  | 
**words** | [WordAlignment] | return | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


