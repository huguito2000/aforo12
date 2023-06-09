<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>vacante1</name>
   <tag></tag>
   <elementGuidId>cd119dfb-858a-4f74-b514-b52fbead4108</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.Token }</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;specialty\&quot;: \&quot;apis\&quot;,\n    \&quot;client\&quot;: {\n        \&quot;clientId\&quot;: \&quot;2c9f9364875cd72f0187a5ff87a10097\&quot;,\n        \&quot;name\&quot;: \&quot;involve\&quot;,\n        \&quot;logo\&quot;: \&quot;https://involve-resources.s3.amazonaws.com/res-involve/company-avatar.png\&quot;\n    },\n    \&quot;numbersVacants\&quot;: \&quot;1\&quot;,\n    \&quot;typeSalary\&quot;: {\n        \&quot;catalogSystemId\&quot;: \&quot;4028e4a986843df5018684517d5d0005\&quot;,\n        \&quot;name\&quot;: \&quot;NO ESPECIFICADO\&quot;,\n        \&quot;type\&quot;: \&quot;typeSalary\&quot;,\n        \&quot;status\&quot;: true,\n        \&quot;keySystem\&quot;: \&quot;MEX\&quot;\n    },\n    \&quot;commissions\&quot;: false,\n    \&quot;confidential\&quot;: false,\n    \&quot;salaryShow\&quot;: false,\n    \&quot;benefitsInvolve\&quot;: [\n        {\n            \&quot;benefitId\&quot;: \&quot;ff8081817b837f74017b838148a00000\&quot;\n        }\n    ],\n    \&quot;city\&quot;: {\n        \&quot;cityId\&quot;: \&quot;2c9f936481969f0cccc996a00e090285\&quot;,\n        \&quot;name\&quot;: \&quot;Miguel Hidalgo\&quot;,\n        \&quot;stateCode\&quot;: \&quot;MX-CMX\&quot;,\n        \&quot;countryCode\&quot;: \&quot;MX\&quot;,\n        \&quot;latitude\&quot;: 19.40824,\n        \&quot;longitude\&quot;: -99.19212,\n        \&quot;state\&quot;: {\n            \&quot;stateId\&quot;: \&quot;2c9f936481969f0bbbb996a00e090009\&quot;,\n            \&quot;name\&quot;: \&quot;Ciudad de México\&quot;,\n            \&quot;countryCode\&quot;: \&quot;MX\&quot;,\n            \&quot;fipsCode\&quot;: \&quot;9\&quot;,\n            \&quot;iso2\&quot;: \&quot;MX-CMX\&quot;,\n            \&quot;latitude\&quot;: 19.28333,\n            \&quot;longitude\&quot;: -99.13333,\n            \&quot;country\&quot;: {\n                \&quot;countryId\&quot;: \&quot;2c9f936481969f0aaaa996a00e090001\&quot;,\n                \&quot;capital\&quot;: \&quot;Mexico City\&quot;,\n                \&quot;currency\&quot;: \&quot;MXN\&quot;,\n                \&quot;currencySymbol\&quot;: \&quot;$\&quot;,\n                \&quot;iso2\&quot;: \&quot;MX\&quot;,\n                \&quot;iso3\&quot;: \&quot;MEX\&quot;,\n                \&quot;latitude\&quot;: 19.4284,\n                \&quot;longitude\&quot;: -99.1276,\n                \&quot;name\&quot;: \&quot;Mexico\&quot;,\n                \&quot;nameNative\&quot;: \&quot;México\&quot;,\n                \&quot;phoneCode\&quot;: \&quot;52\&quot;,\n                \&quot;region\&quot;: \&quot;Americas\&quot;,\n                \&quot;subregion\&quot;: \&quot;Central America\&quot;,\n                \&quot;timezones\&quot;: \&quot;[{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Bahia_Banderas\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Cancun\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-18000,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-05:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;EST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Eastern Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Chihuahua\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Hermosillo\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Matamoros\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Mazatlan\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Merida\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Mexico_City\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Monterrey\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-21600,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-06:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;CST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Central Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Ojinaga\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-25200,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-07:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;MST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Mountain Standard Time (North America\\\&quot;},{\\\&quot;zoneName\\\&quot;:\\\&quot;America/Tijuana\\\&quot;,\\\&quot;gmtOffset\\\&quot;:-28800,\\\&quot;gmtOffsetName\\\&quot;:\\\&quot;UTC-08:00\\\&quot;,\\\&quot;abbreviation\\\&quot;:\\\&quot;PST\\\&quot;,\\\&quot;tzName\\\&quot;:\\\&quot;Pacific Standard Time (North America\\\&quot;}]\&quot;,\n                \&quot;tld\&quot;: \&quot;.mx\&quot;,\n                \&quot;translations\&quot;: \&quot;{\\\&quot;br\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;pt\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;nl\\\&quot;:\\\&quot;Mexico\\\&quot;,\\\&quot;hr\\\&quot;:\\\&quot;Meksiko\\\&quot;,\\\&quot;fa\\\&quot;:\\\&quot;مکزیک\\\&quot;,\\\&quot;de\\\&quot;:\\\&quot;Mexiko\\\&quot;,\\\&quot;es\\\&quot;:\\\&quot;México\\\&quot;,\\\&quot;fr\\\&quot;:\\\&quot;Mexique\\\&quot;,\\\&quot;ja\\\&quot;:\\\&quot;メキシコ\\\&quot;,\\\&quot;it\\\&quot;:\\\&quot;Messico\\\&quot;}\&quot;,\n                \&quot;flagCountry\&quot;: \&quot;https://flagcdn.com/w20/mx.png\&quot;\n            }\n        }\n    },\n     \&quot;modality\&quot;: \&quot;PRESENCIAL\&quot;,\n    \&quot;salaryExactly\&quot;: 2000,\n    \&quot;salaryMinimum\&quot;: 2000,\n    \&quot;position\&quot;: {\n        \&quot;position\&quot;: \&quot;${puesto}\&quot;\n    }\n}&quot;,
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
      <webElementGuid>2cdc8d48-e45a-4d9c-9a6f-c0790cdd858e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.Token }</value>
      <webElementGuid>60a49941-0762-4d9b-a9dc-91f857d3ea95</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/recruiter/vacant</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.puesto</defaultValue>
      <description></description>
      <id>f4424203-4cb3-4dcb-a585-6bf4acb8be74</id>
      <masked>false</masked>
      <name>puesto</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>a5119345-8959-4dd3-9fa2-cec048ee11d2</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
