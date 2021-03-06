<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Create a Photos</name>
   <tag></tag>
   <elementGuidId>91b46e3d-a4b2-46f6-83a7-2f3df045ca30</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;albumId\&quot;: ${albumId},\n    \&quot;title\&quot;: \&quot;${title}\&quot;,\n    \&quot;url\&quot;: \&quot;${url}\&quot;,\n    \&quot;thumbnailUrl\&quot;: \&quot;${thumbnailUrl}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d8e9c6e9-81c0-4a3a-b30c-46b5363c0b3e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/photos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'albumId', 67)
WS.verifyElementPropertyValue(response, 'title', 'Supersemar')
WS.verifyElementPropertyValue(response, 'url', 'https://i.kym-cdn.com/entries/icons/original/000/034/946/Metal-Gear-Rising-Senator-Armstrong.jpg')
WS.verifyElementPropertyValue(response, 'thumbnailUrl', 'https://static.wikia.nocookie.net/firefriendly/images/a/a8/Armstrong.png/revision/latest?cb=20201109022859')
WS.verifyElementPropertyValue(response, 'id', 5001)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
