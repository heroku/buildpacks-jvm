package com.example;

import com.salesforce.functions.jvm.sdk.Context;
import com.salesforce.functions.jvm.sdk.InvocationEvent;
import com.salesforce.functions.jvm.sdk.SalesforceFunction;
import com.salesforce.functions.jvm.sdk.data.Record;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

/**
 * Describe DebuglogsjavaFunction here.
 */
public class DebuglogsjavaFunction implements SalesforceFunction<FunctionInput, FunctionOutput> {
  private static final Logger LOGGER = LoggerFactory.getLogger(DebuglogsjavaFunction.class);

  @Override
  public FunctionOutput apply(InvocationEvent<FunctionInput> event, Context context)
      throws Exception {

    System.out.println("println works");
    LOGGER.info("logging info 1");
    LOGGER.info("logging info 2");
    LOGGER.info("logging info 3");
    LOGGER.debug("logging debug 1");

    List<Account> accounts = new ArrayList<>();

    return new FunctionOutput(accounts);
  }
}
