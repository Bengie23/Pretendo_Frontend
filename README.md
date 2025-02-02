# What is Pretendo Frontend?
Pretendo Frontend is the UI for the Pretendo APP(desktop). It requires Pretendo Backend to run locally in order to work.

# What is Pretendo App for?
Pretendo App is small, but cool http mock server that runs locally by catching http calls from your local network and preventing them to reach out the internet. 

# What does it mean to run locally?
It means that you no longer need to setup any mock calls in your code/tests. 

Before Pretendo
```ruby
# nethttp2.rb
require 'uri'
require 'net/http'
if Rails.env.development?
uri = URI('http://api.nasa.gov/planetary/apod')
else
uri = URI('<your mock url>')
end
params = { :api_key => 'your_api_key' }
uri.query = URI.encode_www_form(params)

res = Net::HTTP.get_response(uri)
puts res.body if res.is_a?(Net::HTTPSuccess)
```

With Pretendo
```ruby
# nethttp2.rb
require 'uri'
require 'net/http'
uri = URI('http://api.nasa.gov/planetary/apod') # same url works for development, test and production

params = { :api_key => 'your_api_key' }
uri.query = URI.encode_www_form(params)

res = Net::HTTP.get_response(uri)
puts res.body if res.is_a?(Net::HTTPSuccess)
```

# How?
By setting up the needed mocks before hand using Pretendo App
![image](https://github.com/user-attachments/assets/7e009ac0-bacf-4267-af82-8d919db2bb94)
When we call this endpoint we get the mock response rather than the real response
![image](https://github.com/user-attachments/assets/c6985675-d85f-4a70-90b7-979c3ab4f1d1)

# Next Steps
TBD...
