use core::convert::Infallible;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub enum Bank {
    Bank0 = 0,
    Bank1 = 1,
}

// 構造体の定義
pub struct MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3> {
    // /CS0
    // チップセレクト 0 入力端子で、プルアップ抵抗を内蔵しています。
    cs0: CS0,

    // CS1
    // チップセレクト 1 入力端子でプルダウン抵抗を内蔵しています。
    // /CS0 端子の状態に拘らず､CS1="H"の時は FOUT 端子が出力可能です。
    // CS1="L"の時､FOUT 端子はハイインピーダンス状態になります。
    cs1: CS1,

    // /RD
    // リードストローブ入力端子です。
    // /RD="L"の時､RTC からのデータリードが可能になります。
    rd: RD,

    // /WR
    // ライトストローブ入力端子です。
    // 立ち上がりエッジで、RTC へデータをライトします。
    wr: WR,

    // A0~A3
    // アドレス入力端子です。
    // 本デバイスへのアクセス時､選択するレジスタアドレスを入力します。
    a0: A0,
    a1: A1,
    a2: A2,
    a3: A3,

    // D0~D3
    // データ入出力端子です。
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
}

// 構造体の初期化関数
impl<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
    MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
{
    pub fn new(
        cs0: CS0,
        cs1: CS1,
        rd: RD,
        wr: WR,
        a0: A0,
        a1: A1,
        a2: A2,
        a3: A3,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
    ) -> Self {
        MyDevice {
            cs0,
            cs1,
            rd,
            wr,
            a0,
            a1,
            a2,
            a3,
            d0,
            d1,
            d2,
            d3,
        }
    }

    fn wait(&mut self, delay: &mut Delay) {
        delay.delay_us(10); // > 150ns
    }
}

impl<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
    MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
where
    CS0: OutputPin<Error = Infallible>,
    CS1: OutputPin<Error = Infallible>,
{
    fn wakeup(&mut self, delay: &mut Delay) {
        // /CS0="L",CS1="H"の時､本デバイスへのアクセスが可能です。
        self.cs0.set_low().unwrap();
        self.wait(delay);
        self.cs1.set_high().unwrap();
        self.wait(delay);
    }

    fn sleep(&mut self, delay: &mut Delay) {
        // CS1 を Low にしておく
        self.cs1.set_low().unwrap();
        self.wait(delay);
    }
}

impl<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
    MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
where
    A0: OutputPin<Error = Infallible>,
    A1: OutputPin<Error = Infallible>,
    A2: OutputPin<Error = Infallible>,
    A3: OutputPin<Error = Infallible>,
{
    fn set_address(&mut self, delay: &mut Delay, addr: u8) {
        self.a0.set_state((addr & 0b1000 == 0b1000).into()).unwrap();
        self.wait(delay);
        self.a1.set_state((addr & 0b0100 == 0b0100).into()).unwrap();
        self.wait(delay);
        self.a2.set_state((addr & 0b0010 == 0b0010).into()).unwrap();
        self.wait(delay);
        self.a3.set_state((addr & 0b0001 == 0b0001).into()).unwrap();
        self.wait(delay);
    }
}

impl<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
    MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
where
    A0: OutputPin<Error = Infallible>,
    A1: OutputPin<Error = Infallible>,
    A2: OutputPin<Error = Infallible>,
    A3: OutputPin<Error = Infallible>,
    D0: InputPin<Error = Infallible>,
{
    fn is_busy(&mut self, delay: &mut Delay) -> Result<bool, Infallible> {
        // データ読む
        let busy = self.d0.is_high().unwrap();
        self.wait(delay);

        Ok(busy)
    }

    pub fn sleep_while_busy(&mut self, delay: &mut Delay) {
        // アドレスを指定
        self.set_address(delay, 0xF);
        while self.is_busy(delay).unwrap() {
            delay.delay_us(240);
        }
    }
}

impl<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
    MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
where
    CS0: OutputPin<Error = Infallible>,
    CS1: OutputPin<Error = Infallible>,
    RD: OutputPin<Error = Infallible>,
    A0: OutputPin<Error = Infallible>,
    A1: OutputPin<Error = Infallible>,
    A2: OutputPin<Error = Infallible>,
    A3: OutputPin<Error = Infallible>,
    D0: InputPin<Error = Infallible>,
    D1: InputPin<Error = Infallible>,
    D2: InputPin<Error = Infallible>,
    D3: InputPin<Error = Infallible>,
{
    pub fn read(&mut self, delay: &mut Delay, addr: u8, data_values: &mut [Option<bool>; 4]) {
        // アドレスを指定
        self.set_address(delay, addr);

        // csアクティブ
        self.wakeup(delay);

        // rdアクティブ
        self.rd.set_low().unwrap();
        self.wait(delay);

        // データ読む
        if let Some(_) = data_values[0] {
            data_values[0] = Some(self.d0.is_high().unwrap());
        }
        if let Some(_) = data_values[1] {
            data_values[1] = Some(self.d1.is_high().unwrap());
        }
        if let Some(_) = data_values[2] {
            data_values[2] = Some(self.d2.is_high().unwrap());
        }
        if let Some(_) = data_values[3] {
            data_values[3] = Some(self.d3.is_high().unwrap());
        }
        self.wait(delay);

        // rdネガティブ
        self.rd.set_high().unwrap();
        self.wait(delay);

        // csネガティブ
        self.sleep(delay);
        self.wait(delay);
    }
}

impl<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
    MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
where
    CS0: OutputPin<Error = Infallible>,
    CS1: OutputPin<Error = Infallible>,
    WR: OutputPin<Error = Infallible>,
    A0: OutputPin<Error = Infallible>,
    A1: OutputPin<Error = Infallible>,
    A2: OutputPin<Error = Infallible>,
    A3: OutputPin<Error = Infallible>,
    D0: OutputPin<Error = Infallible>,
    D1: OutputPin<Error = Infallible>,
    D2: OutputPin<Error = Infallible>,
    D3: OutputPin<Error = Infallible>,
{
    pub fn write(&mut self, delay: &mut Delay, addr: u8, data_values: [Option<bool>; 4]) {
        // アドレスを指定
        self.set_address(delay, addr);

        // csアクティブ
        self.wakeup(delay);

        // wrアクティブ → データバス出力モード
        self.wr.set_low().unwrap();
        self.wait(delay);

        // データ出す
        if let Some(d0) = data_values[0] {
            self.d0.set_state(d0.into()).unwrap();
            self.wait(delay);
        }
        if let Some(d1) = data_values[1] {
            self.d1.set_state(d1.into()).unwrap();
            self.wait(delay);
        }
        if let Some(d2) = data_values[2] {
            self.d2.set_state(d2.into()).unwrap();
            self.wait(delay);
        }
        if let Some(d3) = data_values[3] {
            self.d3.set_state(d3.into()).unwrap();
            self.wait(delay);
        }

        // wrネガティブ → データバス入力モード
        self.wr.set_high().unwrap();
        self.wait(delay);

        // csネガティブ
        self.sleep(delay);
        self.wait(delay);
    }
}
impl<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
    MyDevice<CS0, CS1, RD, WR, A0, A1, A2, A3, D0, D1, D2, D3>
where
    CS0: OutputPin<Error = Infallible>,
    CS1: OutputPin<Error = Infallible>,
    RD: OutputPin<Error = Infallible>,
    WR: OutputPin<Error = Infallible>,
    A0: OutputPin<Error = Infallible>,
    A1: OutputPin<Error = Infallible>,
    A2: OutputPin<Error = Infallible>,
    A3: OutputPin<Error = Infallible>,
    D0: OutputPin<Error = Infallible> + InputPin<Error = Infallible>,
    D1: OutputPin<Error = Infallible>,
    D2: OutputPin<Error = Infallible>,
    D3: OutputPin<Error = Infallible>,
{
    pub fn init(&mut self, delay: &mut Delay, bank: Bank) {
        // 電源初期投入時は "H"レベル ( 非選択状態 ) としてください。
        // > 電源初期投入時の/CS0 端子が " L " レベルとなる場合には、
        // > ご使用前に必ず、一旦 /CS0 端子を " H " レベルにしてから ご使用ください。
        self.cs0.set_high().unwrap();
        self.wait(delay);

        self.sleep(delay);

        let data_values = match bank {
            Bank::Bank0 => [None, Some(true), None, None],
            Bank::Bank1 => [Some(true), None, None, None],
        };
        self.sleep_while_busy(delay);
        self.write(delay, 0xF, data_values);
    }
}
