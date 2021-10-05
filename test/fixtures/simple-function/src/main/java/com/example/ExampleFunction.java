package com.example;

import com.salesforce.functions.jvm.sdk.Context;
import com.salesforce.functions.jvm.sdk.InvocationEvent;
import com.salesforce.functions.jvm.sdk.SalesforceFunction;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ExampleFunction implements SalesforceFunction<String, String> {
  private static final Logger LOGGER = LoggerFactory.getLogger(ExampleFunction.class);

  @Override
  public String apply(InvocationEvent<String> event, Context context) {
    LOGGER.info("logging info 1");
    return new StringBuilder(event.getData()).reverse().toString();
  }
}
