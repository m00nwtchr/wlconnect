import SwiftUI

enum DeviceType: Int{
    case desktop = 0
    case laptop = 1
    case phone = 2
    case tablet = 3
    case tv = 4

    func toSFSymbol() -> String {
        switch self {
            case .desktop:
                return "desktopcomputer"
            case .laptop:
                return "laptopcomputer"
            case .phone:
                return "iphone.gen1"
            case .tablet:
                return "ipad.gen1"
            case .tv:
                return "tv"
        }
    }
}

struct PairedDevice: Identifiable, Equatable {
    var name: String
    var id: String
    var type: DeviceType 
}

struct ConnectedDevice: Identifiable, Equatable {
    var name: String
    var id: String
    var type: DeviceType
    var batteryLevel: Int
    var batteryCharging: Bool
    var batteryLow: Bool
}

func batteryToSFSymbol(device: Binding<ConnectedDevice>) -> String {
    if device.batteryLevel.wrappedValue > 75 {
        return "battery.100"
    } else if device.batteryLevel.wrappedValue > 50 {
        return "battery.75"
    } else if device.batteryLevel.wrappedValue > 25 {
        return "battery.50"
    } else if device.batteryLevel.wrappedValue > 0 {
        return "battery.25"
    } else {
        return "battery.0"
    }
}

@objc public class KConnectSwiftServer: NSObject {
    var view: ContentView
    init(view: ContentView) {
        self.view = view
    }

    @objc func refreshRequested() {
        self.view.refreshDevicesViews()
    }
}

class ContentViewModel: ObservableObject {
    @Published var connected = [ConnectedDevice]()
    @Published var paired = [PairedDevice]()
}

struct ContentView: View {
    @ObservedObject var data = ContentViewModel()
    @State var server: KConnectObjcServer! = nil

    init() {
        self.server = KConnectObjcServer.new(withSwift: KConnectSwiftServer(view: self));
        createMessageCenter()
        let proc = sysctl_ps() as! [NSDictionary]
        let kdeconnectd = proc.first { $0.value(forKey: "proc_name") as! String == "kdeconnectd" }
        if kdeconnectd != nil {
            do {
                try self.refreshConnectedDevices()
                try self.refreshPairedDevices()
            } catch {
                // ignore
            }
        }
    }

    func refreshConnectedDevices() throws {
        guard let connectedArr = getConnectedDevices() as? [NSDictionary] else {
            throw "Error getting connected devices"
        }
        let connectedMapped = try connectedArr.map {
            if let name = $0.value(forKey: "name") as? String,
                let id = $0.value(forKey: "id") as? String,
                let type = $0.value(forKey: "type") as? Int,
                let batteryLevel = $0.value(forKey: "battery_level") as? Int,
                let batteryCharging = $0.value(forKey: "battery_charging") as? Int,
                let batteryUnderThreshold = $0.value(forKey: "battery_under_threshold") as? Int,
                let parsedType = DeviceType(rawValue: type) {
                    let device = ConnectedDevice(
                        name: name,
                        id: id,
                        type: parsedType,
                        batteryLevel: batteryLevel,
                        batteryCharging: batteryCharging == 1,
                        batteryLow: batteryUnderThreshold == 1
                    )
                    return device
            } else {
                throw "Error parsing connected devices"
            }
        }
        self.data.connected = connectedMapped
    }

    func refreshPairedDevices() throws {
        guard let pairedArr = getPairedDevices() as? [NSDictionary] else {
            throw "Error getting paired devices"
        }
        let pairedMapped = try pairedArr.map {
            if let name = $0.value(forKey: "name") as? String,
                let id = $0.value(forKey: "id") as? String,
                let type = $0.value(forKey: "type") as? Int,
                let parsedType = DeviceType(rawValue: type) {
                    let device = PairedDevice(
                        name: name,
                        id: id,
                        type: parsedType
                    )
                    return device
            } else {
                throw "Error parsing paired devices"
            }
        }
        let pairedFiltered = pairedMapped.filter { el in
            !data.connected.contains(where: { $0.id == el.id })
        }
        self.data.paired = pairedFiltered
    }

    func refreshDevicesViews() {
        let proc = sysctl_ps() as! [NSDictionary]
        let kdeconnectd = proc.first { $0.value(forKey: "proc_name") as! String == "kdeconnectd" }
        if kdeconnectd != nil {
            do {
                rebroadcast()
                try self.refreshConnectedDevices()
                try self.refreshPairedDevices()
            } catch {
                UIApplication.shared.alert(body: error.localizedDescription)
            }
        }
    }

	var body: some View {
        NavigationView {
            VStack {
                List {
                    Section(header: Text("Connected devices")) {
                        ForEach(self.$data.connected, id: \.id) { $device in
                            HStack {
                                Image(systemName: device.type.toSFSymbol())
                                if device.batteryCharging {
                                    Image(systemName: "battery.100.bolt").foregroundStyle(.green)
                                } else if device.batteryLow {
                                    Image(systemName: batteryToSFSymbol(device: $device)).foregroundStyle(.red)
                                } else {
                                    Image(systemName: batteryToSFSymbol(device: $device))
                                }
                                Text(device.batteryLevel, format: .percent)
                                VStack(alignment: .leading) {
                                    Text(device.name).lineLimit(1).truncationMode(.tail)
                                    Text(device.id).font(.caption).lineLimit(1).truncationMode(.tail)
                                }
                            }
                        }
                    }
                    Section(header: Text("Paired devices")) {
                        ForEach(self.$data.paired, id: \.id) { $device in
                            HStack {
                                Image(systemName: device.type.toSFSymbol())
                                VStack(alignment: .leading) {
                                    Text(device.name).lineLimit(1).truncationMode(.tail)
                                    Text(device.id).font(.caption).lineLimit(1).truncationMode(.tail)
                                }
                            }
                        }
                    }
                    Section(header: Text("Settings")) {
                        Button("Start daemon (TrollStore only)") { // TODO: Detect if installed via TrollStore
                            let proc = sysctl_ps() as! [NSDictionary]
                            let kdeconnectd = proc.first { $0.value(forKey: "proc_name") as! String == "kdeconnectd" }
                            if kdeconnectd == nil {
                                let bundlePath = Bundle.main.bundlePath
                                let daemonPath = "\(bundlePath)/kdeconnectd"
                                if !FileManager.default.fileExists(atPath: daemonPath) {
                                    UIApplication.shared.alert(body: "Daemon not found")
                                    return
                                }
                                let ret = spawnRoot(daemonPath, [])
                                if ret != 0 {
                                    UIApplication.shared.alert(body: "Error starting daemon: \(ret)")
                                    return
                                }
                            }
                        }
                    }
                }
                .listStyle(InsetGroupedListStyle())
                .refreshable {
                    refreshDevicesViews()
                }
            }
            .navigationTitle("KDE Connect")
        }
	}
}
