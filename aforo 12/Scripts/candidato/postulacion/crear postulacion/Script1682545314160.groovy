import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput
import groovy.json.JsonSlurper as JsonSlurper
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject

WebUI.callTestCase(findTestCase('candidato/registro/formulario 2'), [:], FailureHandling.STOP_ON_FAILURE)

response = WS.sendRequest(findTestObject('candidato/postulacion/postulacion 1'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

GlobalVariable.postulacionId = response.getResponseText()

println(GlobalVariable.postulacionId)

response = WS.sendRequest(findTestObject('candidato/postulacion/cuestionario'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

responseText = response.getResponseText()

println(responseText)

json = new JsonSlurper().parseText(responseText)

json = json.content.questionId

println(json)

codigo = json.sort()

println(codigo)

println(codigo[0])

GlobalVariable.questionId0 = (codigo[0])

println(codigo[1])

GlobalVariable.questionId1 = (codigo[1])

println(codigo[2])

GlobalVariable.questionId2 = (codigo[2])

response = WS.sendRequest(findTestObject('candidato/postulacion/postulacion 2'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

responseText = response.getResponseText()

println(responseText)

response = WS.sendRequest(findTestObject('candidato/postulacion/postulacion 3'))

statusCode = WS.getResponseStatusCode(response)

println(statusCode)

WS.verifyResponseStatusCode(response, 200)

responseText = response.getResponseText()

println(responseText)
