<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>loginC</name>
   <tag></tag>
   <elementGuidId>e1ef84e4-6059-4af5-bb7d-a30a5bb341fd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;email\&quot;: \&quot;${GlobalVariable.email}\&quot;,\n    \&quot;password\&quot;: \&quot;Abcd.1234\&quot;,\n    \&quot;userRol\&quot;: \&quot;CANDIDATE\&quot;\n}&quot;,
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
      <webElementGuid>3180262c-8d54-4e34-9e9c-b6e87d3c1858</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.3</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}.micros.involverh.com.mx/notification/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>ad23de2b-6f9f-41e9-a72c-ac29b2fd9c0c</id>
      <masked>false</masked>
      <name>Token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>47c7a5ec-eafa-405f-9606-8fa6410f2f27</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import java.nio.file.WatchService as WatchService
import javax.xml.bind.annotation.XmlElementDecl.GLOBAL as GLOBAL
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.keyword.builtin.GetResponseStatusCodeKeyword as GetResponseStatusCodeKeyword
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.kms.katalon.entity.global.GlobalVariableEntity as GlobalVariableEntity
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

HttpTextBodyContent body = request.getBodyContent()
println(body.toString())
KeywordUtil.logInfo(response.responseBodyContent)
KeywordLogger log = new KeywordLogger()
log.LogInfo(response.responseBodyContent)
println(response.responseBodyContent)

response = WS.sendRequest(findTestObject('login/login'))

WS.verifyResponseStatusCode(response, 200)

responseText = response.getResponseText()

println(responseText)

def json = new JsonSlurper().parseText(responseText)

println(json.recruiterId)

Map parsed = response.getHeaderFields()

String api = parsed.token

api = api.replace('[', '')
api = api.replace(']', '')
println(api)

GlobalVariable.Token = api





</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
