
#### test results with 50000 orders without amend order

Elapsed: 18.3428344s \
price range (1000..1200) \
qty range (1000..2000) 


#### test results with 10000 orders without amend order

Elapsed: 3.6938319s\
price range (1000..1200) \
qty range (1000..2000) 

#### test results with 10000 orders with amend order

Elapsed: 6.6118689s \
price range (1000..1200) \
qty range (1000..2000) 

### Observation
Very less orders were fulfilled due to the constants issue mentioned below. 

### NOTE: 
constants for MAX_SEQUENCE_ID and ORDER_QUEUE_INIT_CAPACITY were limited to 1000 before the test. 

----

### Tests with increased constants for MAX_SEQUENCE_ID and ORDER_QUEUE_INIT_CAPACITY

#### test results with 50000 orders without amend order

Elapsed: 23.9234207s \
price range (1199..1200) \
qty range (1000..2000) 