<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>addUserGroupToken</name>
   <tag></tag>
   <elementGuidId>792254d2-93fb-45c3-b562-fe10c99069ed</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;v_group_id\&quot;: ${Group_Id},\n  \&quot;v_group_name\&quot;: ${Group_Name},\n  \&quot;v_group_desc\&quot;: ${Group_Desc},\n  \&quot;precedence\&quot;: ${Precedence}\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJqdGkiOiI4MTc3NDc2Ni0yYTViLTQwZmItOWEyZi0zYTk0NzRlZDY4ZTkiLCJpc3MiOiJOWFRBRE1OIiwiYXVkIjoiT0ZTQUEiLCJzdWIiOiJOWFRBRE1OIiwiaWF0IjoxNTY0MzgxNjgwLCJleHAiOjE1NjQzODUyODB9.rXxEPafKexIKGS1WqMBuXVWjvUa6CN9vfG55CoMKN_NJ1ZoTy9NeR5OO9jWp9XgQllSsdkoZSnSHezlk-NCHx_bf1VBrsR-NoR_CMhiPF41qpR1PJLnrUa9jjpcZT4JDKjh9h8LgRLAawrfamI-GmEz-UDgHsiPHdiY6FlVO6G71J1K2nr4sfVEMr_Ahj-Qc3Oa6Iydg-Z_sApAQRMVdwGqpUAgcKpmMQYlXYrktcjPR6Y2xtxYcRdtIAbe50EaEpV3g7Erm_-iab6FL7KqRkuhQKIDsIldsIgbcRM8PurypM1HLwbny4LRQbUCBIOQLRmXSJMvY7aNROKhXKvj1cg</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://tn002.ofs.oracle.com/identity/v1/group/addusergroup</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>40a45d05-1ad8-40b1-98b9-31b72ccf1c07</id>
      <masked>false</masked>
      <name>Group_Id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>97cd74ce-e248-4593-97f3-21689e5a19c5</id>
      <masked>false</masked>
      <name>Group_Name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f2ac9f42-67fd-4a5a-9a89-0a561d4b16e8</id>
      <masked>false</masked>
      <name>Group_Desc</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'payload[0]', 'SMSME.USR_GRP_ADDED')
WS.verifyElementPropertyValue(response, 'payload[0]', 'SMSME.USR_GRP_ADDED')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
