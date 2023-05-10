package com.heroku

import com.twitter.finagle.{Http, Service}
import com.twitter.finagle.http
import com.twitter.util.{Await, Future}

object Server extends App {
  private val service = new Service[http.Request, http.Response] {
    def apply(req: http.Request): Future[http.Response] = {
      val response = http.Response()
      response.setContentString("Hello from Scala!")

      Future.value(response)
    }
  }

  private val server = Http.serve(sys.env.get("PORT").map(":" + _).get, service)
  Await.ready(server)
}
