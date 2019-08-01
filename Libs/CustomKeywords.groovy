
/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */

import java.lang.String


def static "dbConn.DemoMySql.connectDB"(
    	String host	
     , 	String port	
     , 	String service	
     , 	String username	
     , 	String password	) {
    (new dbConn.DemoMySql()).connectDB(
        	host
         , 	port
         , 	service
         , 	username
         , 	password)
}

def static "dbConn.DemoMySql.executeQuery"(
    	String queryString	) {
    (new dbConn.DemoMySql()).executeQuery(
        	queryString)
}

def static "dbConn.DemoMySql.closeDatabaseConnection"() {
    (new dbConn.DemoMySql()).closeDatabaseConnection()
}

def static "dbConn.DemoMySql.execute"(
    	String queryString	) {
    (new dbConn.DemoMySql()).execute(
        	queryString)
}
