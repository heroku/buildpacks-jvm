package com.example;

import com.salesforce.functions.jvm.sdk.Context;
import com.salesforce.functions.jvm.sdk.InvocationEvent;
import com.salesforce.functions.jvm.sdk.SalesforceFunction;

public class ExampleFunction implements SalesforceFunction<String, String> {
  @Override
  public String apply(InvocationEvent<String> event, Context context) {
    return new StringBuilder(event.getData()).reverse().toString();
  }
}
