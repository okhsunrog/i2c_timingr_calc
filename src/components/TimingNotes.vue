<template>
  <div class="mt-6">
    <div class="collapse collapse-arrow bg-base-200">
      <input type="checkbox" />
      <div class="collapse-title text-xl font-medium">Notes</div>
      <div class="collapse-content text-sm italic opacity-75 space-y-4">
        <p>
          <strong>SCLDEL (Data setup time):</strong> Used to generate delay tSCLDEL between SDA edge
          and SCL rising edge. In master mode and slave mode with NOSTRETCH = 0, SCL line is
          stretched low during tSCLDEL. <br />
          tSCLDEL = (SCLDEL+1) × tPRESC <br />
          Note: tSCLDEL is used to generate tSU:DAT timing.
        </p>

        <p>
          <strong>SDADEL (Data hold time):</strong> Used to generate delay tSDADEL between SCL
          falling edge and SDA edge. In master mode and slave mode with NOSTRETCH = 0, SCL line is
          stretched low during tSDADEL. <br />
          tSDADEL = SDADEL × tPRESC <br />
          Note: SDADEL is used to generate tHD:DAT timing.
        </p>

        <p>
          <strong>SCLH (SCL high period):</strong> Used to generate the SCL high period in master
          mode. <br />
          tSCLH = (SCLH+1) × tPRESC <br />
          Note: SCLH is also used to generate tSU:STO and tHD:STA timing.
        </p>

        <p>
          <strong>SCLL (SCL low period):</strong> Used to generate the SCL low period in master
          mode. <br />
          tSCLL = (SCLL+1) × tPRESC <br />
          Note: SCLL is also used to generate tBUF and tSU:STA timings.
        </p>
      </div>
    </div>
  </div>
</template>
